use alloy_primitives::{ Address, Bytes, U256 };
use revm::primitives::{ Account, Bytecode, HashMap };

use crate::{ error::BackendError, memory::UnmanagedVector };

use super::{ vtable::Db, StateDB };

pub struct StateDB<'r> {
    pub db: &'r Db,
}

impl<'r> StateDB<'r> {
    pub fn new(db: &'r Db) -> Self {
        StateDB { db }
    }
}
