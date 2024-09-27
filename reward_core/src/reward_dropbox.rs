use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::discriminators::Discriminators;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct RewardDropbox {}

impl Discriminator for RewardDropbox {
    const DISCRIMINATOR: u8 = Discriminators::RewardDropbox as u8;
}

impl RewardDropbox {
    /// Returns the seeds for the PDA
    pub fn seeds(ncn: &Pubkey) -> Vec<Vec<u8>> {
        vec![b"REWARD_DROPBOX".as_ref().to_vec(), ncn.to_bytes().to_vec()]
    }

    pub fn find_program_address(program_id: &Pubkey, ncn: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }
}
