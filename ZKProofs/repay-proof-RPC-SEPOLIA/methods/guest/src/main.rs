
#![allow(unused_doc_comments)]
#![no_main]


use proof_core::core::{LiquidityLookResult, Stake, U256};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};
use alloy_sol_types::{sol, SolValue};
use alloy_primitives::{address, Address};
use risc0_steel::{config::{ChainSpec, ETH_SEPOLIA_CHAIN_SPEC} , ethereum::EthEvmInput, Contract};
// use revm::primitives::SpecId;

risc0_zkvm::guest::entry!(main);




sol! {
    interface IYTP {
        function getLend(address user, uint lendId) public  view  returns (uint);
    }
}



const CONTRACT: Address = address!("E501F32748Ea5c70f2FA617db70EB3Aa063FA16f");
/// Address of the caller.
const CALLER: Address = address!("f08A50178dfcDe18524640EA6618a1f965821715");



fn main() {
    let params: EthEvmInput = env::read();
    let user: Address = env::read();
    let lendId: U256 = env::read();
    // let mut  ZK_SYNC_SPEC: ChainSpec =  ChainSpec::new_single(0x12c,SpecId::LATEST  ,EIP1559_CONSTANTS_DEFAULT);

    
    let CALL: IYTP::getLendCall = IYTP::getLendCall {
        user,
        lendId,
    };
    let env = params.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    
    let contract = Contract::new(CONTRACT, &env);
    let returns = contract.call_builder(&CALL).from(CALLER).call();
    // assert!(returns._0 == U256::from(0));
    env::commit_slice(&env.block_commitment().abi_encode());
}

