use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use shank::ShankType;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Pod,
    BorshDeserialize,
    BorshSerialize,
    Zeroable,
    ShankType,
    Default,
)]
#[repr(C)]
pub struct MerkleRoot {
    pub root: [u8; 32],
}

impl MerkleRoot {
    pub fn is_empty(&self) -> bool {
        for byte in self.root {
            if byte > 0 {
                return false;
            }
        }

        return true;
    }
}
