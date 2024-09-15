use std::sync::Arc;

use revm::{
    db::EmptyDB,
    primitives::{EthereumWiring, SpecId},
    Evm, EvmBuilder, EvmHandler,
};

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
}
