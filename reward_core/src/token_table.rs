use std::u64;

use bytemuck::{Pod, Zeroable};
use jito_bytemuck::types::PodU64;
use shank::ShankType;
use solana_program::pubkey::Pubkey;

use crate::error::RewardCoreError;

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod, Default)]
#[repr(C)]
pub struct TokenTable {
    pub table: [TokenEntry; 32],
}

impl TokenTable {
    pub const MAX_TABLE_ENTRIES: usize = 32;

    pub fn new() -> Self {
        Self {
            table: [TokenEntry::default(); Self::MAX_TABLE_ENTRIES],
        }
    }

    pub fn entry_count(&self) -> usize {
        self.table.iter().filter(|entry| !entry.is_empty()).count()
    }

    pub fn find_value(&self, mint: &Pubkey) -> Option<u64> {
        self.table
            .iter()
            .find(|entry| entry.mint == *mint)
            .map(|entry| entry.value.into())
    }

    pub fn set_value(&mut self, mint: &Pubkey, value: u64) -> Result<(), RewardCoreError> {
        let entry = self
            .table
            .iter_mut()
            .find(|entry| entry.mint == *mint || entry.is_empty());

        match entry {
            Some(entry) => {
                entry.value = value.into();
                if entry.mint == Pubkey::default() {
                    entry.mint = *mint;
                }
            }
            None => return Err(RewardCoreError::NoMoreTableSlots),
        }

        Ok(())
    }
}

pub type EpochPriceTable = TokenTable;
pub type StakeTable = TokenTable;

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct TokenEntry {
    pub value: PodU64,
    pub mint: Pubkey,
}

impl TokenEntry {
    pub const EMPTY_ENTRY_VALUE: u64 = u64::MAX;

    pub fn new(value: u64, mint: Pubkey) -> Self {
        Self {
            value: PodU64::from(value),
            mint,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value == PodU64::from(Self::EMPTY_ENTRY_VALUE)
    }
}

impl Default for TokenEntry {
    fn default() -> Self {
        Self {
            value: PodU64::from(TokenEntry::EMPTY_ENTRY_VALUE),
            mint: Pubkey::default(),
        }
    }
}

pub type EpochPriceTableEntry = TokenEntry;
pub type StakeEntry = TokenEntry;
