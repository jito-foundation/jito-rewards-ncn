use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RewardError {
    #[error("VaultSlashUnderflow")]
    VaultSlashUnderflow = 1000,
}

impl<T> DecodeError<T> for RewardError {
    fn type_of() -> &'static str {
        "jito::reward"
    }
}

impl From<RewardError> for ProgramError {
    fn from(e: RewardError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<RewardError> for u64 {
    fn from(e: RewardError) -> Self {
        e as Self
    }
}

impl From<RewardError> for u32 {
    fn from(e: RewardError) -> Self {
        e as Self
    }
}
