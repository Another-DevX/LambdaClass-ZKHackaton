// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use proof_core::core::{Address,Repay, RepayProofResult , U256};
use risc0_zkvm::{default_prover, ExecutorEnv};




fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

      
    

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    // For example:
    // let id = U256::from(1);
    // let amount = U256::from(10);
    // let address = Address::random();
    // let input = Repay {id, amount, address };

    // let env = ExecutorEnv::builder()
    //     .write(&input)
    //     .unwrap()
    //     .build()
    //     .unwrap();

    // let prover = default_prover();

    // println!("Running the guest with the constructed input:");

    // let prove_info = prover.prove(env, REPAY_PROOF_GUEST_ELF).unwrap();

    // let receipt = prove_info.receipt;

    // let _output: RepayProofResult = receipt.journal.decode().unwrap();

    // println!("{:?}", _output);

    // receipt.verify(REPAY_PROOF_GUEST_ID).unwrap();
}
