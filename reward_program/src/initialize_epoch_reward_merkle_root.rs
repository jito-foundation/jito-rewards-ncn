use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_restaking_core::{config::Config as RestakingConfig, ncn::Ncn};
use jito_restaking_program::ID as RESTAKING_PROGRAM_ID;
use jito_reward_core::epoch_reward_merkle_root::EpochRewardMerkleRoot;
use jito_vault_core::config::Config as RewardConfig;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

pub fn process_initialize_epoch_reward_merkle_root(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [reward_config, restaking_config, ncn, epoch_reward_merkle_root, payer, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let restaking_program = RESTAKING_PROGRAM_ID;

    // Account Checks
    load_system_account(epoch_reward_merkle_root, true)?;
    load_signer(payer, true)?;
    load_system_program(system_program)?;

    Ncn::load(&restaking_program, ncn, false)?;
    RestakingConfig::load(&restaking_program, restaking_config, false)?;
    RewardConfig::load(program_id, reward_config, false)?;

    let ncn_epoch = {
        let restaking_config_account_data = restaking_config.data.borrow();
        let restaking_config_account =
            RestakingConfig::try_from_slice_unchecked(&restaking_config_account_data)?;
        EpochRewardMerkleRoot::epoch(Clock::get()?.slot, restaking_config_account.epoch_length())
            .unwrap()
    };

    let (
        epoch_reward_merkle_root_pubkey,
        epoch_reward_merkle_root_bump,
        mut epoch_reward_merkle_root_seeds,
    ) = EpochRewardMerkleRoot::find_program_address(program_id, ncn.key, ncn_epoch);
    epoch_reward_merkle_root_seeds.push(vec![epoch_reward_merkle_root_bump]);

    if epoch_reward_merkle_root_pubkey.ne(epoch_reward_merkle_root.key) {
        msg!("Reward Merkle Root account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing epoch reward merkle root (slot {}) at address {}",
        ncn_epoch,
        epoch_reward_merkle_root_pubkey
    );

    create_account(
        payer,
        epoch_reward_merkle_root,
        system_program,
        &restaking_program,
        &Rent::get()?,
        EpochRewardMerkleRoot::size(),
        &epoch_reward_merkle_root_seeds,
    )?;

    let mut epoch_reward_merkle_root_data = epoch_reward_merkle_root.try_borrow_mut_data()?;
    epoch_reward_merkle_root_data[0] = EpochRewardMerkleRoot::DISCRIMINATOR;
    let epoch_reward_merkle_root =
        EpochRewardMerkleRoot::try_from_slice_unchecked_mut(&mut epoch_reward_merkle_root_data)?;
    *epoch_reward_merkle_root = EpochRewardMerkleRoot::new(*ncn.key, ncn_epoch);

    Ok(())
}
