use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::discriminators::Discriminators;

// Empty struct to mark the epoch reward distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct EpochRewardDistributionMarker {}

impl Discriminator for EpochRewardDistributionMarker {
    const DISCRIMINATOR: u8 = Discriminators::EpochRewardDistributionMarker as u8;
}

impl EpochRewardDistributionMarker {
    /// Returns the seeds for the PDA
    pub fn seeds(
        epoch_reward_merkle_root: &Pubkey,
        mint: &Pubkey,
        recipient: &Pubkey,
        ncn_epoch: u64,
    ) -> Vec<Vec<u8>> {
        vec![
            b"EPOCH_REWARD_DISTRIBUTION_MARKER".as_ref().to_vec(),
            epoch_reward_merkle_root.to_bytes().to_vec(),
            mint.to_bytes().to_vec(),
            recipient.to_bytes().to_vec(),
            ncn_epoch.to_le_bytes().to_vec(),
        ]
    }

    /// Returns the PDA
    pub fn find_program_address(
        program_id: &Pubkey,
        epoch_reward_merkle_root: &Pubkey,
        mint: &Pubkey,
        recipient: &Pubkey,
        ncn_epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(epoch_reward_merkle_root, mint, recipient, ncn_epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }
}
