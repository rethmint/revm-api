mod compiler;
mod error;
mod interface;
mod memory;
mod states;
mod utils;
mod types;

mod evm {
    pub mod v1 {
        pub mod block {
            include!(concat!(env!("OUT_DIR"), "/evm.v1.block.rs"));
        }
        pub mod transaction {
            include!(concat!(env!("OUT_DIR"), "/evm.v1.transaction.rs"));
        }
        pub mod result {
            include!(concat!(env!("OUT_DIR"), "/evm.v1.result.rs"));
        }
    }
}

pub use interface::*;
pub use evm::*;
