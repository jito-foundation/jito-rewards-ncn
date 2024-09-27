use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::{discriminators::Discriminators, merkle_root::MerkleRoot, token_table::StakeTable};

/// Voting ticket for the epoch reward merkle root
#[derive(Debug, Clone, Copy, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct EpochRewardMerkleRootTicket {
    pub ncn: Pubkey,
    pub operator: Pubkey,
    pub ncn_slot: u64,
    pub root: MerkleRoot,
    pub stake_table: StakeTable,
}

impl Discriminator for EpochRewardMerkleRootTicket {
    const DISCRIMINATOR: u8 = Discriminators::EpochRewardMerkleRootTicket as u8;
}

impl EpochRewardMerkleRootTicket {
    /// Create a new vault account
    pub fn new(ncn: Pubkey, operator: Pubkey, ncn_slot: u64) -> Self {
        Self {
            ncn,
            operator,
            ncn_slot,
            root: MerkleRoot::default(),
            stake_table: StakeTable::default(),
        }
    }

    /// Returns the seeds for the PDA
    pub fn seeds(ncn: &Pubkey, operator: &Pubkey, ncn_epoch: u64) -> Vec<Vec<u8>> {
        vec![
            b"EPOCH_MERKLE_ROOT_TICKET".as_ref().to_vec(),
            ncn.to_bytes().to_vec(),
            operator.to_bytes().to_vec(),
            ncn_epoch.to_le_bytes().to_vec(),
        ]
    }

    /// Returns the PDA
    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        operator: &Pubkey,
        ncn_epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, operator, ncn_epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    pub fn has_voted(&self) -> bool {
        !self.root.is_empty()
    }
}
