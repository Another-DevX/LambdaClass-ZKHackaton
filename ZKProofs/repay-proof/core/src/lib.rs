pub use ethers_core::types::{Address, U256};

pub use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Repay {
    pub id: U256,
    pub amount: U256,
    pub address: Address,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RepayProofResult {
    pub repay_state: Digest,
}

impl RepayProofResult {
    pub fn new(repay_state: Digest) -> Self {
        Self { repay_state }
    }
    
}