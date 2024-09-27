use borsh::{BorshDeserialize, BorshSerialize};
use jito_reward_core::merkle_root::MerkleRoot;
use shank::ShankInstruction;

#[rustfmt::skip]
#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum RewardInstruction {


// [config, admin, ncn, restaking_program, system_program]

    /// Initializes global configuration
    #[account(0, writable, name = "config")]
    #[account(1, name = "ncn")]
    #[account(2, writable, signer, name = "admin")]
    #[account(3, name = "system_program")]
    InitializeConfig{
        valid_voting_slots: u64,
        slots_before_closing_marker_accounts: u64,
    },

    /// initializes a reward merkle root for the given epoch
    /// reward_config, restaking_config, ncn, epoch_reward_merkle_root, payer, system_program
    #[account(0, writable, name = "reward_config")]
    #[account(1, writable, name = "restaking_config")]
    #[account(2, name = "ncn")]
    #[account(3, writable, name = "epoch_reward_merkle_root")]
    #[account(4, writable, signer, name = "payer")]
    #[account(5, name = "system_program")]
    InitializeEpochRewardMerkleRoot,

    /// initializes a reward merkle root ticket for the given epoch
    #[account(0, writable, signer, name = "admin")]
    InitializeEpochRewardMerkleRootTicket,

    /// initializes a reward merkle root ticket for the given epoch
    #[account(0, writable, signer, name = "admin")]
    UpdateTicketStake,

    /// initializes a reward merkle root ticket for the given epoch
    #[account(0, writable, signer, name = "admin")]
    SubmitTicket {
        root: MerkleRoot,
    },

    /// Deposits rewards to EpochRewardMerkleRoot
    #[account(0, writable, name = "epoch_reward_merkle_root")]
    #[account(1, writable, signer, name = "depositor")]
    DepositRewards,

    /// Creates a slash ticket if slashing conditions are met
    #[account(0, writable, name = "slash_ticket")]
    #[account(1, signer, name = "slasher")]
    Slash,

    /// Transfers rewards from RewardDropbox to the latest EpochRewardMerkleRoot
    #[account(0, writable, name = "reward_dropbox")]
    #[account(1, writable, name = "latest_epoch_reward_merkle_root")]
    DropboxToLatest,

    /// Transfers rewards from a delinquent epoch to the current rewards
    #[account(0, writable, name = "delinquent_epoch_reward_merkle_root")]
    #[account(1, writable, name = "current_epoch_reward_merkle_root")]
    DelinquentToLatest,

    /// Distributes rewards from a valid EpochRewardMerkleRoot
    #[account(0, writable, name = "epoch_reward_merkle_root")]
    #[account(1, writable, name = "distribution_marker")]
    DistributeCrank,

    /// Closes marker accounts older than a specified number of slots
    #[account(0, writable, name = "marker_account")]
    CloseMarkerAccounts,
}
