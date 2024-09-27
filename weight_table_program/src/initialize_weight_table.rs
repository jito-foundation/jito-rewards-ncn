use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_restaking_core::ncn::Ncn;
use jito_restaking_program::ID as RESTAKING_PROGRAM_ID;
use jito_reward_core::reward_config::RewardConfig;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

pub fn process_initialize_weight_table(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    //TODO

    Ok(())
}
