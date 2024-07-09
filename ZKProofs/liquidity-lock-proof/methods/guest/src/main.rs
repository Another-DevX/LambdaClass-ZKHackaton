use proof_core::core::{LiquidityLookResult, Stake, U256};
use risc0_zkvm::{
    guest::env,
    serde::to_vec,
    sha::{Impl, Sha256},
};
use alloy_sol_types::{sol, SolValue};
use alloy_primitives::{address, Address};
use risc0_steel::{config::{ChainSpec, EIP1559_CONSTANTS_DEFAULT} , ethereum::EthEvmInput, Contract};
use revm::primitives::SpecId;



sol! {
    interface IYTP {
        function getLend(address user, uint256 lendId) public  view  returns (uint256 value);
    }
}



const CONTRACT: Address = address!("6765E325c59376B706b7fFc5bc12664580438FBB");
/// Address of the caller.
const CALLER: Address = address!("f08A50178dfcDe18524640EA6618a1f965821715");



fn main() {
    let params: EthEvmInput = env::read();
    let user: Address = env::read();
    let lending_id: u64 = env::read();
    let mut  ZK_SYNC_SPEC: ChainSpec =  ChainSpec::new_single(0x12c,SpecId::LATEST  ,EIP1559_CONSTANTS_DEFAULT);

    
    let mut CALL: IYTP::getLendCall = IYTP::getLendCall {
        user,
        lendId: U256::from(lending_id),
    };
    let env = params.into_env().with_chain_spec(&ZK_SYNC_SPEC);

    
    let contract = Contract::new(CONTRACT, &env);
    let response = contract.call_builder(&CALL).call();
    assert!(response.value == U256::from(0));
    env::commit_slice(&env.block_commitment().abi_encode());
}

