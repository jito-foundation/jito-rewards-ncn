use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_reward_core::merkle_root::MerkleRoot;
use jito_vault_core::{config::Config, vault::Vault};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_submit_ticket(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    root: MerkleRoot,
) -> ProgramResult {
    // Add code here

    todo!();
}
