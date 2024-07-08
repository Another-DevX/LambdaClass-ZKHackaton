use proof_core::core::{LiquidityLookResult, Stake};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};
use alloy_sol_types::{sol, SolValue};



sol! {
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
}



fn main() {
    let params: Stake = env::read();

    let output = LiquidityLookResult::new(*Impl::hash_words(&to_vec(&params).unwrap()));

    env::commit(&output);
}
