use core::{RepayProofResult, Repay};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};

fn main() {
    let params: Repay = env::read();

    let output = RepayProofResult::new(*Impl::hash_words(&to_vec(&params).unwrap()));

    env::commit(&output);
}
