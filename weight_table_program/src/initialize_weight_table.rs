use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_restaking_core::{config::Config, ncn::Ncn};
use jito_weight_table_core::{error::WeightTableError, weight_table::WeightTable};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

/// Initializes a Weight Table
pub fn process_initialize_weight_table(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [restaking_config, ncn, weight_table, weight_table_admin, restaking_program_id, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    Config::load(restaking_program_id.key, restaking_config, false)?;
    let ncn_epoch_length = {
        let config_data = restaking_config.data.borrow();
        let config = Config::try_from_slice_unchecked(&config_data)?;
        config.epoch_length()
    };

    Ncn::load(restaking_program_id.key, ncn, false)?;
    let ncn_weight_table_admin = {
        //TODO switch to weight table admin when that is merged
        let ncn_data = ncn.data.borrow();
        let ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;
        ncn.admin
    };

    load_system_account(weight_table, true)?;
    load_signer(weight_table_admin, true)?;
    load_system_program(system_program)?;

    if restaking_program_id.key.ne(&jito_restaking_program::id()) {
        msg!("Incorrect restaking program ID");
        return Err(ProgramError::InvalidAccountData);
    }

    if ncn_weight_table_admin.ne(&weight_table_admin.key) {
        msg!("Vault update delegations ticket is not at the correct PDA");
        return Err(WeightTableError::IncorrectWeightTableAdmin.into());
    }

    let current_slot = Clock::get()?.slot;
    let ncn_epoch = current_slot
        .checked_div(ncn_epoch_length)
        .ok_or(WeightTableError::DenominatorIsZero)?;

    let (weight_table_pubkey, weight_table_bump, mut weight_table_seeds) =
        WeightTable::find_program_address(program_id, ncn.key, ncn_epoch);
    weight_table_seeds.push(vec![weight_table_bump]);

    if weight_table_pubkey.ne(weight_table.key) {
        msg!("Incorrect weight table PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing Weight Table {} for NCN: {} at epoch: {}",
        weight_table.key,
        ncn.key,
        ncn_epoch
    );
    create_account(
        weight_table_admin,
        weight_table,
        system_program,
        program_id,
        &Rent::get()?,
        8_u64.checked_add(size_of::<WeightTable>() as u64).unwrap(),
        &weight_table_seeds,
    )?;

    let mut weight_table_data = weight_table.try_borrow_mut_data()?;
    weight_table_data[0] = WeightTable::DISCRIMINATOR;
    let weight_table_account = WeightTable::try_from_slice_unchecked_mut(&mut weight_table_data)?;
    *weight_table_account = WeightTable::new(*ncn.key, ncn_epoch, weight_table_bump);

    Ok(())
}
