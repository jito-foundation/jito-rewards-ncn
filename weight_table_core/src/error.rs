use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WeightTableError {
    #[error("No more table slots available")]
    NoMoreTableSlots = 0x2000,
    #[error("Zero in the denominator")]
    DenominatorIsZero = 0x2100,
    #[error("Overflow")]
    ArithmeticOverflow = 0x2101,
    #[error("Modulo Overflow")]
    ModuloOverflow = 0x2102,

    #[error("Incorrect weight table admin")]
    IncorrectWeightTableAdmin = 0x2200,
}

impl<T> DecodeError<T> for WeightTableError {
    fn type_of() -> &'static str {
        "jito::weight_table"
    }
}

impl From<WeightTableError> for ProgramError {
    fn from(e: WeightTableError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<WeightTableError> for u64 {
    fn from(e: WeightTableError) -> Self {
        e as Self
    }
}

impl From<WeightTableError> for u32 {
    fn from(e: WeightTableError) -> Self {
        e as Self
    }
}
