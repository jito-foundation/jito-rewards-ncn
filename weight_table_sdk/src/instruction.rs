use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[rustfmt::skip]
#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum WeightTableInstruction {

    /// Initializes global configuration
    #[account(0, name = "restaking_config")]
    #[account(1, name = "ncn")]
    #[account(2, writable, signer, name = "weight_table")]
    #[account(3, writable, signer, name = "weight_table_admin")]
    #[account(4, name = "restaking_program_id")]
    #[account(5, name = "system_program")]
    InitializeWeightTable{
        first_slot_of_ncn_epoch: Option<u64>,
    },

    /// Updates the weight table
    #[account(0, name = "ncn")]
    #[account(1, writable, name = "weight_table")]
    #[account(2, signer, name = "weight_table_admin")]
    #[account(3, name = "restaking_program_id")]
    UpdateWeightTable{
        ncn_epoch: u64,
        weight_numerator: u64,
        weight_denominator: u64,
    },

    #[account(0, name = "ncn")]
    #[account(1, writable, name = "weight_table")]
    #[account(2, signer, name = "weight_table_admin")]
    #[account(3, name = "restaking_program_id")]
    FinalizeWeightTable{
        ncn_epoch: u64,
    },

}
