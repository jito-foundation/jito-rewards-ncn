use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{discriminators::WEIGHT_TABLE_DISCRIMINATOR, error::WeightTableError, weight::Weight};

// PDA'd ["WEIGHT_TABLE", NCN, NCN_EPOCH_SLOT]
#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct WeightTable {
    /// The NCN on-chain program is the signer to create and update this account,
    /// this pushes the responsibility of managing the account to the NCN program.
    pub ncn: Pubkey,

    /// The NCN epoch for which the weight table is valid
    pub ncn_epoch: PodU64,

    /// Slot weight table was created
    slot_created: PodU64,

    /// Slot weight table was finalized
    slot_finalized: PodU64,

    /// Bump seed for the PDA
    pub bump: u8,

    /// Reserved space
    reserved: [u8; 128],

    /// The weight table
    pub table: [WeightEntry; 32],
}

impl Discriminator for WeightTable {
    const DISCRIMINATOR: u8 = WEIGHT_TABLE_DISCRIMINATOR;
}

impl WeightTable {
    pub const MAX_TABLE_ENTRIES: usize = 32;
    pub const NOT_FINALIZED: u64 = u64::MAX;

    pub fn new(ncn: Pubkey, ncn_epoch: u64, slot_created: u64, bump: u8) -> Self {
        Self {
            ncn,
            ncn_epoch: PodU64::from(ncn_epoch),
            slot_created: PodU64::from(slot_created),
            slot_finalized: PodU64::from(Self::NOT_FINALIZED),
            bump,
            reserved: [0; 128],
            table: [WeightEntry::default(); Self::MAX_TABLE_ENTRIES],
        }
    }

    pub fn seeds(ncn: &Pubkey, ncn_epoch: u64) -> Vec<Vec<u8>> {
        Vec::from_iter(
            [
                b"WEIGHT_TABLE".to_vec(),
                ncn.to_bytes().to_vec(),
                ncn_epoch.to_le_bytes().to_vec(),
            ]
            .iter()
            .cloned(),
        )
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        ncn_epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, ncn_epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    pub fn entry_count(&self) -> usize {
        self.table.iter().filter(|entry| !entry.is_empty()).count()
    }

    pub fn find_weight(&self, mint: &Pubkey) -> Option<Weight> {
        self.table
            .iter()
            .find(|entry| entry.mint == *mint)
            .map(|entry| entry.weight)
    }

    pub fn set_weight(&mut self, mint: &Pubkey, weight: Weight) -> Result<(), WeightTableError> {
        let entry = self
            .table
            .iter_mut()
            .find(|entry| entry.mint == *mint || entry.is_empty());

        match entry {
            Some(entry) => {
                entry.weight = weight.into();

                if entry.mint == Pubkey::default() {
                    entry.mint = *mint;
                }
            }
            None => return Err(WeightTableError::NoMoreTableSlots),
        }

        Ok(())
    }

    pub fn slot_created(&self) -> u64 {
        self.slot_created.into()
    }

    pub fn slot_finalized(&self) -> u64 {
        self.slot_finalized.into()
    }

    pub fn finalized(&self) -> bool {
        self.slot_finalized != PodU64::from(Self::NOT_FINALIZED)
    }

    pub fn finalize(&mut self, current_slot: u64) {
        self.slot_finalized = PodU64::from(current_slot);
    }

    pub fn load(
        program_id: &Pubkey,
        weight_table: &AccountInfo,
        ncn: &AccountInfo,
        ncn_epoch: u64,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if weight_table.owner.ne(program_id) {
            msg!("Weight table account is not owned by the program");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if weight_table.data_is_empty() {
            msg!("Weight table is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !weight_table.is_writable {
            msg!("Weight table account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if weight_table.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Weight table account has an incorrect discriminator");
            return Err(ProgramError::InvalidAccountData);
        }
        let expected_pubkey = Self::find_program_address(program_id, ncn.key, ncn_epoch).0;
        if weight_table.key.ne(&expected_pubkey) {
            msg!("Weight table incorrect PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct WeightEntry {
    pub mint: Pubkey,
    pub weight: Weight,
}

impl WeightEntry {
    pub fn new(mint: Pubkey, weight: Weight) -> Self {
        Self { weight, mint }
    }

    pub fn is_empty(&self) -> bool {
        self.weight.denominator() == 0 || self.mint.eq(&Pubkey::default())
    }
}

#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;

    use super::*;

    #[test]
    fn test_weight_table_new() {
        let ncn = Pubkey::new_unique();
        let table = WeightTable::new(ncn, 0, 0, 0);
        assert_eq!(table.entry_count(), 0);
    }

    #[test]
    fn test_weight_table_entry_count() {
        let ncn = Pubkey::new_unique();

        let mut table = WeightTable::new(ncn, 0, 0, 0);
        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();

        table
            .set_weight(&mint1, Weight::new(1, 2).unwrap())
            .unwrap();
        assert_eq!(table.entry_count(), 1);

        table
            .set_weight(&mint2, Weight::new(3, 4).unwrap())
            .unwrap();
        assert_eq!(table.entry_count(), 2);
    }

    #[test]
    fn test_weight_table_find_weight() {
        let ncn = Pubkey::new_unique();

        let mut table = WeightTable::new(ncn, 0, 0, 0);
        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();

        table
            .set_weight(&mint1, Weight::new(1, 2).unwrap())
            .unwrap();
        table
            .set_weight(&mint2, Weight::new(3, 4).unwrap())
            .unwrap();

        assert_eq!(table.find_weight(&mint1), Some(Weight::new(1, 2).unwrap()));
        assert_eq!(table.find_weight(&mint2), Some(Weight::new(3, 4).unwrap()));
        assert_eq!(table.find_weight(&Pubkey::new_unique()), None);
    }

    #[test]
    fn test_weight_table_set_weight() {
        let ncn = Pubkey::new_unique();

        let mut table = WeightTable::new(ncn, 0, 0, 0);
        let mint = Pubkey::new_unique();

        // Set initial weight
        table.set_weight(&mint, Weight::new(1, 2).unwrap()).unwrap();
        assert_eq!(table.find_weight(&mint), Some(Weight::new(1, 2).unwrap()));

        // Update weight
        table.set_weight(&mint, Weight::new(3, 4).unwrap()).unwrap();
        assert_eq!(table.find_weight(&mint), Some(Weight::new(3, 4).unwrap()));

        // Fill table and test error
        for _ in 0..WeightTable::MAX_TABLE_ENTRIES - 1 {
            table
                .set_weight(&Pubkey::new_unique(), Weight::new(1, 1).unwrap())
                .unwrap();
        }
        let result = table.set_weight(&Pubkey::new_unique(), Weight::new(1, 1).unwrap());
        assert!(matches!(result, Err(WeightTableError::NoMoreTableSlots)));
    }

    #[test]
    fn test_weight_entry_new() {
        let mint = Pubkey::new_unique();
        let weight = Weight::new(1, 2).unwrap();
        let entry = WeightEntry::new(mint, weight);
        assert_eq!(entry.mint, mint);
        assert_eq!(entry.weight, weight);
    }

    #[test]
    fn test_weight_entry_is_empty() {
        let empty_entry = WeightEntry::default();
        assert!(empty_entry.is_empty());

        let mint = Pubkey::new_unique();
        let weight = Weight::new(1, 2).unwrap();
        let non_empty_entry = WeightEntry::new(mint, weight);
        assert!(!non_empty_entry.is_empty());
    }

    #[test]
    fn test_weight_table_finalize() {
        let mut weight_table = WeightTable::new(Pubkey::new_unique(), 0, 0, 0);

        assert!(!weight_table.finalized());
        assert_eq!(weight_table.slot_finalized(), WeightTable::NOT_FINALIZED);

        weight_table.finalize(0);
        assert!(weight_table.finalized());
    }
}
