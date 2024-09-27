use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::pubkey::Pubkey;

use crate::{discriminators::Discriminators, merkle_root::MerkleRoot, token_table::TokenTable};

/// The vault is responsible for holding tokens and minting VRT tokens
/// based on the amount of tokens deposited.
/// It also contains several administrative functions for features inside the vault.
#[derive(Debug, Clone, Copy, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct EpochRewardMerkleRoot {
    pub ncn: Pubkey,
    pub ncn_epoch: PodU64,
    pub reward_payout_count: PodU64, // How many times the `distribute_crank` was called
    pub roots: [MerkleRootEntry; 32],
}

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod, Default)]
#[repr(C)]
pub struct MerkleRootEntry {
    pub root: MerkleRoot,
    pub stake: TokenTable,
}

impl Discriminator for EpochRewardMerkleRoot {
    const DISCRIMINATOR: u8 = Discriminators::EpochRewardMerkleRoot as u8;
}

impl EpochRewardMerkleRoot {
    pub const MAX_ROOTS: usize = 32;

    pub fn size() -> u64 {
        8_u64
            .checked_add(std::mem::size_of::<EpochRewardMerkleRoot>() as u64)
            .unwrap()
    }

    /// Create a new vault account
    pub fn new(ncn: Pubkey, ncn_epoch: u64) -> Self {
        Self {
            ncn,
            ncn_epoch: PodU64::from(ncn_epoch),
            reward_payout_count: PodU64::default(),
            roots: [MerkleRootEntry::default(); Self::MAX_ROOTS],
        }
    }

    pub fn epoch(current_slot: u64, epoch_length: u64) -> Option<u64> {
        current_slot.checked_div(epoch_length)
    }

    /// Returns the seeds for the PDA
    pub fn seeds(ncn: &Pubkey, ncn_epoch: u64) -> Vec<Vec<u8>> {
        vec![
            b"EPOCH_MERKLE_ROOT".as_ref().to_vec(),
            ncn.to_bytes().to_vec(),
            ncn_epoch.to_le_bytes().to_vec(),
        ]
    }

    /// Returns the PDA address for the merkle root
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

    pub fn is_voting_done(&self, current_slot: u64, valid_voting_slots: u64) -> bool {
        let voting_cutoff = u64::from(self.ncn_epoch)
            .checked_add(valid_voting_slots)
            .unwrap();

        voting_cutoff <= current_slot
    }

    pub fn get_highest_voted_root(&self, price_table: &TokenTable) -> Option<MerkleRoot> {
        todo!()
    }
}
