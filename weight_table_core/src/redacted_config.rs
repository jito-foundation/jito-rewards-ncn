use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::pubkey::Pubkey;

use crate::{error::WeightTableError, weight::Weight};

// May not go with the config approach

// One config per ncn

#[derive(
    Debug, Clone, Copy, Zeroable, ShankType, Pod, Default, AccountDeserialize, ShankAccount,
)]
#[repr(C)]
pub struct Config {
    pub ncn: Pubkey,
    pub admin: Pubkey,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = 3;
}

impl Config {
    pub fn new(ncn: Pubkey, admin: Pubkey) -> Self {
        Self { ncn, admin }
    }

    pub fn seeds(ncn: &Pubkey) -> Vec<Vec<u8>> {
        Vec::from_iter(
            [b"CONFIG".to_vec(), ncn.to_bytes().to_vec()]
                .iter()
                .cloned(),
        )
    }

    pub fn find_program_address(program_id: &Pubkey, ncn: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }
}
