use proof_core::core::{LiquidityLookResult, Stake};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};
use alloy_sol_types::{sol, SolValue};


sol! {
    /// YTP balance function signature.
    interface IYTP {
        function getBalance(address user) external view returns (uint);
    }
}


const CALL: IYTP::balanceOfCall = IYTP::balanceOfCall {
    account: address!("9737100D2F42a196DE56ED0d1f6fF598a250E7E4"),
};

const CONTRACT: Address = address!("Fb5C5dF867c3798710A7135a1D75CF0c6eb45132");




fn main() {
    let params: Stake = env::read();

    let output = LiquidityLookResult::new(*Impl::hash_words(&to_vec(&params).unwrap()));

    env::commit(&output);
}
