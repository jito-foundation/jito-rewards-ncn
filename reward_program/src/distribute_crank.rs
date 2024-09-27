use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_vault_core::{config::Config, vault::Vault};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_distribute_crank(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Add code here

    todo!();
}
