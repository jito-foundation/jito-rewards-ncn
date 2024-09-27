mod close_marker_accounts;
mod delinquent_to_latest;
mod deposit_rewards;
mod distribute_crank;
mod dropbox_to_latest;
mod initialize_config;
mod initialize_epoch_reward_merkle_root;
mod initialize_epoch_reward_merkle_root_ticket;
mod restaking_helpers;
mod slash;
mod submit_ticket;
mod update_ticket_stake;

use borsh::BorshDeserialize;
use const_str_to_pubkey::str_to_pubkey;
use jito_reward_sdk::instruction::RewardInstruction;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

use crate::{
    close_marker_accounts::process_close_marker_accounts,
    delinquent_to_latest::process_delinquent_to_latest, deposit_rewards::process_deposit_rewards,
    distribute_crank::process_distribute_crank, dropbox_to_latest::process_dropbox_to_latest,
    initialize_config::process_initialize_config,
    initialize_epoch_reward_merkle_root::process_initialize_epoch_reward_merkle_root,
    initialize_epoch_reward_merkle_root_ticket::process_initialize_epoch_reward_merkle_root_ticket,
    slash::process_slash, submit_ticket::process_submit_ticket,
    update_ticket_stake::process_update_ticket_stake,
};

declare_id!(str_to_pubkey(env!("REWARD_PROGRAM_ID")));

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Jito's Reward NCN",
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

    let instruction = RewardInstruction::try_from_slice(instruction_data)?;

    match instruction {
        // ------------------------------------------
        // Initialization
        // ------------------------------------------
        RewardInstruction::InitializeConfig {
            valid_voting_slots,
            slots_before_closing_marker_accounts,
        } => {
            msg!("Instruction: InitializeConfig");
            process_initialize_config(
                program_id,
                accounts,
                valid_voting_slots,
                slots_before_closing_marker_accounts,
            )
        }
        RewardInstruction::InitializeEpochRewardMerkleRoot => {
            msg!("Instruction: InitializeEpochRewardMerkleRoot");
            process_initialize_epoch_reward_merkle_root(program_id, accounts)
        }
        RewardInstruction::InitializeEpochRewardMerkleRootTicket => {
            msg!("Instruction: InitializeEpochRewardMerkleRootTicket");
            process_initialize_epoch_reward_merkle_root_ticket(program_id, accounts)
        }
        // ------------------------------------------
        // Reward Operations
        // ------------------------------------------
        RewardInstruction::DepositRewards => {
            msg!("Instruction: DepositRewards");
            process_deposit_rewards(program_id, accounts)
        }
        RewardInstruction::UpdateTicketStake => {
            msg!("Instruction: UploadAndVote");
            process_update_ticket_stake(program_id, accounts)
        }
        RewardInstruction::SubmitTicket { root } => {
            msg!("Instruction: UploadAndVote");
            process_submit_ticket(program_id, accounts, root)
        }
        RewardInstruction::Slash => {
            msg!("Instruction: Slash");
            process_slash(program_id, accounts)
        }
        RewardInstruction::DropboxToLatest => {
            msg!("Instruction: DropboxToLatest");
            process_dropbox_to_latest(program_id, accounts)
        }
        RewardInstruction::DelinquentToLatest => {
            msg!("Instruction: DelinquentToLatest");
            process_delinquent_to_latest(program_id, accounts)
        }
        RewardInstruction::DistributeCrank => {
            msg!("Instruction: DistributeCrank");
            process_distribute_crank(program_id, accounts)
        }
        RewardInstruction::CloseMarkerAccounts => {
            msg!("Instruction: CloseMarkerAccounts");
            process_close_marker_accounts(program_id, accounts)
        }
    }
}
