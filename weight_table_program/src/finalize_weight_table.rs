use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_restaking_core::ncn::Ncn;
use jito_weight_table_core::{error::WeightTableError, weight_table::WeightTable};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

/// Initializes a Weight Table
pub fn process_finalize_weight_table(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ncn_epoch: u64,
) -> ProgramResult {
    let [ncn, weight_table, weight_table_admin, restaking_program_id] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Ncn::load(restaking_program_id.key, ncn, false)?;
    let ncn_weight_table_admin = {
        //TODO switch to weight table admin when that is merged
        let ncn_data = ncn.data.borrow();
        let ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;
        ncn.admin
    };

    load_signer(weight_table_admin, true)?;
    WeightTable::load(program_id, weight_table, ncn, ncn_epoch, true)?;

    if restaking_program_id.key.ne(&jito_restaking_program::id()) {
        msg!("Incorrect restaking program ID");
        return Err(ProgramError::InvalidAccountData);
    }

    if ncn_weight_table_admin.ne(&weight_table_admin.key) {
        msg!("Vault update delegations ticket is not at the correct PDA");
        return Err(WeightTableError::IncorrectWeightTableAdmin.into());
    }

    let mut weight_table_data = weight_table.try_borrow_mut_data()?;
    let weight_table_account = WeightTable::try_from_slice_unchecked_mut(&mut weight_table_data)?;

    let current_slot = Clock::get()?.slot;
    weight_table_account.finalize(current_slot);

    Ok(())
}
