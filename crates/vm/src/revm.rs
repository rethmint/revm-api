use std::sync::Arc;

use revm::{
    db::EmptyDB,
    handler::mainnet::create,
    primitives::{EthereumWiring, SpecId, Transaction},
    Evm, EvmBuilder, EvmHandler,
};
use revm_interpreter::{CreateInputs, CreateScheme, Gas};
use types::{BackendError, MessageOutput};

#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct Revm {
    pub revm: Arc<Evm<'static, EthereumWiring<EmptyDB, ()>>>,
}

impl Revm {
    pub fn new() -> Self {
        let builder = EvmBuilder::default()
            .with_default_db()
            .with_default_ext_ctx();

        let mainnet =
            EvmHandler::<'_, EthereumWiring<EmptyDB, ()>>::mainnet_with_spec(SpecId::CANCUN);
        let builder = builder.with_handler(mainnet);

        let evm = builder.build();

        Self {
            revm: Arc::new(evm),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize(&mut self, tx_env: &impl Transaction) -> Result<MessageOutput, BackendError> {
        let gas_limit = Gas::new(u64::MAX);

        let create_inputs = CreateInputs {
            init_code: tx_env.data().clone(),
            gas_limit: gas_limit.remaining(),
            caller: *tx_env.caller(),
            scheme: CreateScheme::Create,
            value: *tx_env.value(),
        };

        let create_result = create(&mut context, Box::new(create_inputs)).unwrap();

        let contract_address = match create_result {
            FrameOrResult::Result(CreateOutcome { address, .. }) => address.unwrap(), // Extract address
            FrameOrResult::Frame(_) => panic!("Unexpected frame result during contract creation"),
        };

        // Now, call a function on the deployed contract
        let function_selector = get_function_selector("init_genesis");
        let args: Vec<Vec<u8>> = vec![/* Arguments for the contract function */];

        // Encode the function call with arguments
        let call_data = abi::encode_call(function_selector, args)?;

        // Set up call inputs for calling the contract
        let call_inputs = CallInputs {
            contract: contract_address,
            input: Bytes::from(call_data),
            gas_limit: gas_limit.remaining(),
            ..Default::default() // Additional fields for value, sender, etc.
        };

        // Call the function using REVM's call function
        let call_result = call::<EvmWiringT, SPEC>(&mut context, Box::new(call_inputs))?;

        // No need for session cleanup, REVM handles state changes and gas tracking natively
    }
}
