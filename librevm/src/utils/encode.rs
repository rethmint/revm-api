use alloy_primitives::Address;
use revm::primitives::{ ExecutionResult, HaltReason, OutOfGasError, SuccessReason };

pub fn build_flat_buffer(result: ExecutionResult) -> Vec<u8> {
    todo!()
}

pub fn ivec_to_u64(ivec: &sled::IVec) -> Option<u64> {
    ivec.as_ref().try_into().ok().map(u64::from_be_bytes)
}
