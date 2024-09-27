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

pub fn process_initialize_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    valid_voting_slots: u64,
    slots_before_closing_marker_accounts: u64,
) -> ProgramResult {
    let [config, ncn, admin, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let restaking_program = RESTAKING_PROGRAM_ID;

    // Account Checks
    load_system_account(config, true)?;
    load_signer(admin, true)?;
    load_system_program(system_program)?;

    Ncn::load(&restaking_program, ncn, false)?;

    let (config_pubkey, config_bump, mut config_seeds) =
        RewardConfig::find_program_address(program_id, ncn.key);
    config_seeds.push(vec![config_bump]);

    if config_pubkey.ne(config.key) {
        msg!("Config account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    // Create Account
    msg!("Initializing config at address {}", config.key);
    create_account(
        admin,
        config,
        system_program,
        program_id,
        &Rent::get()?,
        RewardConfig::size(),
        &config_seeds,
    )?;

    let mut config_data = config.try_borrow_mut_data()?;
    config_data[0] = RewardConfig::DISCRIMINATOR;
    let config = RewardConfig::try_from_slice_unchecked_mut(&mut config_data)?;
    *config = RewardConfig::new(
        ncn.key,
        admin.key,
        valid_voting_slots,
        slots_before_closing_marker_accounts,
    );

    Ok(())
}
