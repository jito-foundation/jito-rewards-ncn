use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RewardCoreError {
    #[error("No more table slots available")]
    NoMoreTableSlots = 0x2000,
}

impl<T> DecodeError<T> for RewardCoreError {
    fn type_of() -> &'static str {
        "jito::vault"
    }
}

impl From<RewardCoreError> for ProgramError {
    fn from(e: RewardCoreError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<RewardCoreError> for u64 {
    fn from(e: RewardCoreError) -> Self {
        e as Self
    }
}

impl From<RewardCoreError> for u32 {
    fn from(e: RewardCoreError) -> Self {
        e as Self
    }
}
