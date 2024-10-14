mod finalize_weight_table;
mod initialize_weight_table;
mod update_weight_table;

use borsh::BorshDeserialize;
use const_str_to_pubkey::str_to_pubkey;
use jito_weight_table_core::instruction::WeightTableInstruction;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

use crate::{
    finalize_weight_table::process_finalize_weight_table,
    initialize_weight_table::process_initialize_weight_table,
    update_weight_table::process_update_weight_table,
};

declare_id!(str_to_pubkey(env!("WEIGHT_TABLE_ID")));

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Jito's Weight Table Program",
    project_url: "https://jito.network/",
    contacts: "email:team@jito.network",
    policy: "https://github.com/jito-foundation/jito-rewards-ncn",
    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/jito-foundation/jito-rewards-ncn"
}

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = WeightTableInstruction::try_from_slice(instruction_data)?;

    match instruction {
        // ------------------------------------------
        // Initialization
        // ------------------------------------------
        WeightTableInstruction::InitializeWeightTable {
            first_slot_of_ncn_epoch,
        } => {
            msg!("Instruction: InitializeWeightTable");
            process_initialize_weight_table(program_id, accounts, first_slot_of_ncn_epoch)
        }
        // ------------------------------------------
        // Update
        // ------------------------------------------
        WeightTableInstruction::UpdateWeightTable {
            ncn_epoch,
            weight_numerator,
            weight_denominator,
        } => {
            msg!("Instruction: UpdateWeightTable");
            process_update_weight_table(
                program_id,
                accounts,
                ncn_epoch,
                weight_numerator,
                weight_denominator,
            )
        }
        // ------------------------------------------
        // Finalization
        // ------------------------------------------
        WeightTableInstruction::FinalizeWeightTable { ncn_epoch } => {
            msg!("Instruction: FinalizeWeightTable");
            process_finalize_weight_table(program_id, accounts, ncn_epoch)
        }
    }
}
