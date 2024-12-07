use alloy_primitives::{ Address, Bytes, U256 };
use revm::primitives::{ Account, Bytecode, HashMap };

use crate::{ error::BackendError, memory::UnmanagedVector };

use super::StateDB;

pub trait Getter {
    #[allow(dead_code)]
    fn get_storage_root(&self, address: Address) -> Result<Vec<u8>, BackendError>;

    fn get_code(&self, address: Address) -> Result<Vec<u8>, BackendError>;

    fn get_code_hash(&self, address: Address) -> Result<Vec<u8>, BackendError>;

    fn get_state(&self, address: Address, slot_hash: Vec<u8>) -> Result<Vec<u8>, BackendError>;

    fn get_balance(&self, address: Address) -> Result<Vec<u8>, BackendError>;

    fn get_nonce(&self, address: Address) -> Result<Vec<u8>, BackendError>;
}

impl Getter for StateDB {
    fn get_storage_root(&self, address: Address) -> Result<Vec<u8>, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_storage_root)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn get_code(&self, address: Address) -> Result<Bytes, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_code)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn get_code_hash(&self, address: Address) -> Result<U256, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_code_hash)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn get_state(&self, address: Address, slot_hash: U256) -> Result<U256, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_state)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(slot_hash)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn get_balance(&self, address: Address) -> Result<U256, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_balance)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn get_nonce(&self, address: Address) -> Result<U256, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .get_nonce)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut output as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let output = output.consume().unwrap_or_else(|| default.to_vec());

        let default = || {
            format!("Failed to read a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }
}
