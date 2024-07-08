use core::{LiquidityLookResult, Stake};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};

fn main() {
    let params: Stake = env::read();

    let output = LiquidityLookResult::new(*Impl::hash_words(&to_vec(&params).unwrap()));

    env::commit(&output);
}
