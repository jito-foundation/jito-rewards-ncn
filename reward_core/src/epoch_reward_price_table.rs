use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::{discriminators::Discriminators, token_table::EpochPriceTable};

/// Epoch Price Table - contains the normalized price of each mint for a given epoch
/// This needs to be verified before any voting can occur
#[derive(Debug, Clone, Copy, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct EpochRewardPriceTable {
    pub ncn: Pubkey,

    pub update_authority: Pubkey,

    pub ncn_slot: PodU64,

    pub verified: u8,

    pub table: EpochPriceTable,
}

impl Discriminator for EpochRewardPriceTable {
    const DISCRIMINATOR: u8 = Discriminators::EpochPriceTable as u8;
}

impl EpochRewardPriceTable {
    pub const MAX_TABLE_ENTRIES: usize = 32;

    /// Create new Price Table
    pub fn new(ncn: Pubkey, update_authority: Pubkey, ncn_slot: u64) -> Self {
        Self {
            ncn,
            update_authority,
            ncn_slot: PodU64::from(ncn_slot),
            verified: 0,
            table: EpochPriceTable::new(),
        }
    }

    /// Returns the seeds for the PDA
    pub fn seeds(ncn: &Pubkey, ncn_epoch: u64) -> Vec<Vec<u8>> {
        vec![
            b"EPOCH_PRICE_TABLE".as_ref().to_vec(),
            ncn.to_bytes().to_vec(),
            ncn_epoch.to_le_bytes().to_vec(),
        ]
    }

    /// Returns the PDA
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
}
