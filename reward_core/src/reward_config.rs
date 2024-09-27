use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::discriminators::Discriminators;

/// The vault is responsible for holding tokens and minting VRT tokens
/// based on the amount of tokens deposited.
/// It also contains several administrative functions for features inside the vault.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct RewardConfig {
    pub ncn: Pubkey,

    pub admin: Pubkey,

    pub valid_voting_slots: PodU64, // amount of slots voting is valid for after an epoch ends
    pub slots_before_closing_marker_accounts: PodU64, // amount of slots before a marker account can be closed
}

impl Discriminator for RewardConfig {
    const DISCRIMINATOR: u8 = Discriminators::RewardConfig as u8;
}

impl RewardConfig {
    pub fn size() -> u64 {
        8_u64
            .checked_add(std::mem::size_of::<RewardConfig>() as u64)
            .unwrap()
    }

    /// Create a new vault account
    pub fn new(
        ncn: &Pubkey,
        admin: &Pubkey,
        valid_voting_slots: u64,
        slots_before_closing_marker_accounts: u64,
    ) -> Self {
        Self {
            ncn: *ncn,
            admin: *admin,
            //TODO determine default values
            valid_voting_slots: PodU64::from(valid_voting_slots),
            slots_before_closing_marker_accounts: PodU64::from(
                slots_before_closing_marker_accounts,
            ),
        }
    }

    /// Returns the seeds for the PDA
    pub fn seeds(ncn: &Pubkey) -> Vec<Vec<u8>> {
        vec![b"REWARD_CONFIG".as_ref().to_vec(), ncn.to_bytes().to_vec()]
    }

    /// Returns the PDA address for the merkle root
    pub fn find_program_address(program_id: &Pubkey, ncn: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    pub fn load(
        program_id: &Pubkey,
        ncn: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, ncn).0)
        {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
