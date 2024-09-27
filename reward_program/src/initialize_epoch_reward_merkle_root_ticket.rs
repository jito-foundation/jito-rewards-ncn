use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::{load_signer, load_system_account, load_system_program};
use jito_restaking_program::ID as RESTAKING_PROGRAM_ID;
use jito_vault_core::{config::Config, vault::Vault};
use jito_vault_program::ID as VAULT_PROGRAM_ID;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::restaking_helpers::check_ncn_vault_operator_active;

pub fn process_initialize_epoch_reward_merkle_root_ticket(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [vault_config_info, vault_info, ncn_info, operator_info, ncn_operator_state_info, ncn_vault_ticket_info, operator_vault_ticket_info, vault_ncn_ticket_info, vault_operator_delegation_info, ncn_vault_slasher_ticket_info, vault_ncn_slasher_ticket_info, epoch_reward_merkle_root, epoch_reward_merkle_root_ticket, slasher, admin, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let restaking_program = RESTAKING_PROGRAM_ID;
    let vault_program = VAULT_PROGRAM_ID;

    // Account Checks
    load_system_account(epoch_reward_merkle_root_ticket, true)?;
    load_signer(admin, true)?;
    load_system_program(system_program)?;

    check_ncn_vault_operator_active(
        &restaking_program,
        &vault_program,
        slasher,
        vault_config_info,
        vault_info,
        ncn_info,
        operator_info,
        ncn_operator_state_info,
        ncn_vault_ticket_info,
        operator_vault_ticket_info,
        vault_ncn_ticket_info,
        vault_operator_delegation_info,
        ncn_vault_slasher_ticket_info,
        vault_ncn_slasher_ticket_info,
    )?;

    todo!();
}
