use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_restaking_core::{
    ncn::Ncn, ncn_operator_state::NcnOperatorState,
    ncn_vault_slasher_ticket::NcnVaultSlasherTicket, ncn_vault_ticket::NcnVaultTicket,
    operator::Operator, operator_vault_ticket::OperatorVaultTicket,
};
use jito_vault_core::{
    config::Config as VaultConfig, vault::Vault,
    vault_ncn_slasher_operator_ticket::VaultNcnSlasherOperatorTicket,
    vault_ncn_slasher_ticket::VaultNcnSlasherTicket, vault_ncn_ticket::VaultNcnTicket,
    vault_operator_delegation::VaultOperatorDelegation,
};
use jito_vault_sdk::error::VaultError;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

pub fn get_ncn_epoch_from_config(
    slot: u64,
    vault_config: &VaultConfig,
) -> Result<u64, ProgramError> {
    let epoch_length = vault_config.epoch_length();
    get_ncn_epoch(slot, epoch_length)
}

pub fn get_ncn_epoch(slot: u64, epoch_length: u64) -> Result<u64, ProgramError> {
    let ncn_epoch = slot
        .checked_div(epoch_length)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(ncn_epoch)
}

#[allow(clippy::too_many_arguments)]
pub fn check_ncn_vault_operator_active(
    restaking_program: &Pubkey,
    vault_program: &Pubkey,
    slasher: &AccountInfo,
    vault_config_info: &AccountInfo,
    vault_info: &AccountInfo,
    ncn_info: &AccountInfo,
    operator_info: &AccountInfo,
    ncn_operator_state_info: &AccountInfo,
    ncn_vault_ticket_info: &AccountInfo,
    operator_vault_ticket_info: &AccountInfo,
    vault_ncn_ticket_info: &AccountInfo,
    vault_operator_delegation_info: &AccountInfo,
    ncn_vault_slasher_ticket_info: &AccountInfo,
    vault_ncn_slasher_ticket_info: &AccountInfo,
) -> ProgramResult {
    // Load the vault config
    VaultConfig::load(vault_program, vault_config_info, false)?;
    let vault_config_data = vault_config_info.data.borrow();
    let vault_config = VaultConfig::try_from_slice_unchecked(&vault_config_data)?;

    // Load vault
    Vault::load(vault_program, vault_info, true)?;
    let mut vault_data = vault_info.data.borrow_mut();
    let vault = Vault::try_from_slice_unchecked_mut(&mut vault_data)?;

    // Load NCN
    Ncn::load(restaking_program, ncn_info, false)?;

    // Load Operator
    Operator::load(restaking_program, operator_info, false)?;

    // Load NCN Operator State
    NcnOperatorState::load(
        restaking_program,
        ncn_operator_state_info,
        ncn_info,
        operator_info,
        false,
    )?;
    let ncn_operator_state_data = ncn_operator_state_info.data.borrow();
    let ncn_operator_state = NcnOperatorState::try_from_slice_unchecked(&ncn_operator_state_data)?;

    // Load NCN Vault Ticket
    NcnVaultTicket::load(
        restaking_program,
        ncn_vault_ticket_info,
        ncn_info,
        vault_info,
        false,
    )?;
    let ncn_vault_ticket_data = ncn_vault_ticket_info.data.borrow();
    let ncn_vault_ticket = NcnVaultTicket::try_from_slice_unchecked(&ncn_vault_ticket_data)?;

    // Load Operator Vault Ticket
    OperatorVaultTicket::load(
        restaking_program,
        operator_vault_ticket_info,
        operator_info,
        vault_info,
        false,
    )?;
    let operator_vault_ticket_data = operator_vault_ticket_info.data.borrow();
    let operator_vault_ticket =
        OperatorVaultTicket::try_from_slice_unchecked(&operator_vault_ticket_data)?;

    // Load Vault NCN Ticket
    VaultNcnTicket::load(
        vault_program,
        vault_ncn_ticket_info,
        vault_info,
        ncn_info,
        false,
    )?;
    let vault_ncn_ticket_data = vault_ncn_ticket_info.data.borrow();
    let vault_ncn_ticket = VaultNcnTicket::try_from_slice_unchecked(&vault_ncn_ticket_data)?;

    // Load Vault Operator Delegation
    VaultOperatorDelegation::load(
        vault_program,
        vault_operator_delegation_info,
        vault_info,
        operator_info,
        true,
    )?;
    let mut vault_operator_delegation_data = vault_operator_delegation_info.data.borrow_mut();
    let vault_operator_delegation =
        VaultOperatorDelegation::try_from_slice_unchecked_mut(&mut vault_operator_delegation_data)?;

    // Load slasher
    load_signer(slasher, false)?;

    // Load NCN Vault Slasher Ticket
    NcnVaultSlasherTicket::load(
        restaking_program,
        ncn_vault_slasher_ticket_info,
        ncn_info,
        vault_info,
        slasher,
        false,
    )?;
    let ncn_vault_slasher_ticket_data = ncn_vault_slasher_ticket_info.data.borrow();
    let ncn_vault_slasher_ticket =
        NcnVaultSlasherTicket::try_from_slice_unchecked(&ncn_vault_slasher_ticket_data)?;

    // Load Vault NCN Slasher Ticket
    VaultNcnSlasherTicket::load(
        vault_program,
        vault_ncn_slasher_ticket_info,
        vault_info,
        ncn_info,
        slasher,
        false,
    )?;
    let vault_ncn_slasher_ticket_data = vault_ncn_slasher_ticket_info.data.borrow();
    let vault_ncn_slasher_ticket =
        VaultNcnSlasherTicket::try_from_slice_unchecked(&vault_ncn_slasher_ticket_data)?;

    let current_slot = Clock::get()?.slot;
    let epoch_length = vault_config.epoch_length();

    // All ticket states shall be active or cooling down
    check_states_active_or_cooling_down(
        ncn_operator_state,
        operator_vault_ticket,
        vault_ncn_ticket,
        ncn_vault_ticket,
        vault_ncn_slasher_ticket,
        ncn_vault_slasher_ticket,
        current_slot,
        epoch_length,
    )?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn check_states_active_or_cooling_down(
    ncn_operator_state: &NcnOperatorState,
    operator_vault_ticket: &OperatorVaultTicket,
    vault_ncn_ticket: &VaultNcnTicket,
    ncn_vault_ticket: &NcnVaultTicket,
    vault_ncn_slasher_ticket: &VaultNcnSlasherTicket,
    ncn_vault_slasher_ticket: &NcnVaultSlasherTicket,
    slot: u64,
    epoch_length: u64,
) -> ProgramResult {
    if !vault_ncn_slasher_ticket
        .state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("Vault NCN slasher ticket is not active or in cooldown");
        return Err(VaultError::VaultNcnSlasherTicketUnslashable.into());
    }
    if !ncn_vault_slasher_ticket
        .state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("NCN vault slasher ticket is not active or in cooldown");
        return Err(VaultError::NcnVaultSlasherTicketUnslashable.into());
    }
    if !ncn_operator_state
        .ncn_opt_in_state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("NCN opt-in to operator is not active or in cooldown");
        return Err(VaultError::NcnOperatorStateUnslashable.into());
    }
    if !ncn_operator_state
        .operator_opt_in_state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("Operator opt-in to NCN is not active or in cooldown");
        return Err(VaultError::NcnOperatorStateUnslashable.into());
    }
    if !operator_vault_ticket
        .state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("Operator vault ticket is not active or in cooldown");
        return Err(VaultError::OperatorVaultTicketUnslashable.into());
    }
    if !vault_ncn_ticket
        .state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("Vault NCN ticket is not active or in cooldown");
        return Err(VaultError::VaultNcnTicketUnslashable.into());
    }
    if !ncn_vault_ticket
        .state
        .is_active_or_cooldown(slot, epoch_length)
    {
        msg!("NCN vault ticket is not active or in cooldown");
        return Err(VaultError::NcnVaultTicketUnslashable.into());
    }
    Ok(())
}
