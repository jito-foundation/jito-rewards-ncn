use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::pubkey::Pubkey;

use crate::{error::WeightTableError, weight::Weight};

// PDA'd ["WEIGHT_TABLE", NCN, NCN_EPOCH_SLOT]
#[derive(
    Debug, Clone, Copy, Zeroable, ShankType, Pod, Default, AccountDeserialize, ShankAccount,
)]
#[repr(C)]
pub struct WeightTable {
    /// The NCN on-chain program is the signer to create and update this account,
    /// this pushes the responsibility of managing the account to the NCN program.
    pub ncn: Pubkey,

    /// The slot starting the NCN epoch, the epoch length is determined by the restaking program config.
    pub ncn_epoch_slot: PodU64,

    /// Anything non-zero means the table is finalized and cannot be updated.
    pub finalized: u8,

    pub table: [WeightEntry; 32],
}

impl Discriminator for WeightTable {
    const DISCRIMINATOR: u8 = 2;
}

impl WeightTable {
    pub const MAX_TABLE_ENTRIES: usize = 32;
    pub const NOT_FINALIZED: u8 = 0;
    pub const FINALIZED: u8 = 0xFF;

    pub fn new(ncn: Pubkey, ncn_epoch_slot: u64) -> Self {
        Self {
            ncn,
            ncn_epoch_slot: PodU64::from(ncn_epoch_slot),
            finalized: Self::NOT_FINALIZED,
            table: [WeightEntry::default(); Self::MAX_TABLE_ENTRIES],
        }
    }

    pub fn seeds(ncn: &Pubkey, ncn_epoch_slot: u64) -> Vec<Vec<u8>> {
        Vec::from_iter(
            [
                b"WEIGHT_TABLE".to_vec(),
                ncn.to_bytes().to_vec(),
                ncn_epoch_slot.to_le_bytes().to_vec(),
            ]
            .iter()
            .cloned(),
        )
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        ncn_epoch_slot: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, ncn_epoch_slot);
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

    pub fn is_finalized(&self, current_slot: u64, epoch_length: u64) -> bool {
        let finalized = self.finalized != Self::NOT_FINALIZED;
        let epoch_over = current_slot >= u64::from(self.ncn_epoch_slot) + epoch_length;

        finalized || epoch_over
    }

    pub fn finalize(&mut self) {
        self.finalized = Self::FINALIZED;
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
        let table = WeightTable::new(ncn, 0);
        assert_eq!(table.entry_count(), 0);
    }

    #[test]
    fn test_weight_table_entry_count() {
        let ncn = Pubkey::new_unique();

        let mut table = WeightTable::new(ncn, 0);
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

        let mut table = WeightTable::new(ncn, 0);
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

        let mut table = WeightTable::new(ncn, 0);
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
}
