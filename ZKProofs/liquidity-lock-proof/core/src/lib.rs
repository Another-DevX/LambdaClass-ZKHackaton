pub use ethers_core::types::{Address, U256};

pub use risc0_zkp::core::digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Stake {
    pub amount: U256,
    pub address: Address,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiquidityLookResult {
    pub liquidity_look_state: Digest,
}

impl LiquidityLookResult {
    pub fn new(liquidity_look_state: Digest) -> Self {
        Self { liquidity_look_state }
    }
    
}