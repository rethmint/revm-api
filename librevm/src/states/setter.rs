use alloy_primitives::{ Address, Bytes, U256 };
use revm::primitives::{ Account, Bytecode, HashMap };

use crate::{ error::BackendError, memory::UnmanagedVector };

use super::StateDB;

pub trait Setter {
    #[allow(dead_code)]
    fn add_balance(&mut self, address: Address, amount: Vec<u8>) -> Result<(), BackendError>;

    fn sub_balance(&mut self, address: Address, amount: Vec<u8>) -> Result<(), BackendError>;

    fn set_balance(&mut self, address: Address, balance: Vec<u8>) -> Result<(), BackendError>;

    fn set_nonce(&mut self, address: Address, nonce: u64) -> Result<(), BackendError>;

    fn set_code(&mut self, address: Address, code: Vec<u8>) -> Result<(), BackendError>;

    fn set_state(
        &mut self,
        address: Address,
        slot_hash: Vec<u8>,
        value: Vec<u8>
    ) -> Result<(), BackendError>;

    fn set_storage(&mut self, address: Address, storage_input: Vec<u8>) -> Result<(), BackendError>;

    fn self_destruct(&mut self, address: Address) -> Result<(), BackendError>;

    fn commit(&mut self, block_number: u64, delete_empty_objects: bool) -> Result<(), BackendError>;
}

impl Setter for StateDB {
    fn add_balance(&mut self, address: Address, amount: U256) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .add_balance)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(address)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn sub_balance(&mut self, address: Address, amount: U256) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .sub_balance)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(address)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(output)
    }

    fn set_balance(&mut self, address: Address, balance: U256) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .set_balance)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(balance)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn set_nonce(&mut self, address: Address, nonce: u64) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .set_nonce)(
                self.db.state,
                U8SliceView::new(Some(address)),
                nonce,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn set_code(&mut self, address: Address, code: Bytecode) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .set_code)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(code)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn set_state(
        &mut self,
        address: Address,
        slot_hash: U256,
        value: U256
    ) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .set_state)(
                self.db.state,
                U8SliceView::new(Some(address)),
                U8SliceView::new(Some(slot_hash)),
                U8SliceView::new(Some(value)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn set_storage(
        &mut self,
        address: Address,
        storage_input: HashMap<Address, Account>
    ) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let mut storage_input = UnmanagedVector::new(Some(storage_input));
        let go_error: GoError = (self.db.vtable
            .set_storage)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut storage_input as *mut UnmanagedVector,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn self_destruct(&mut self, address: Address) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .self_destruct)(
                self.db.state,
                U8SliceView::new(Some(address)),
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to write a key in the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }

    fn commit(
        &mut self,
        block_number: u64,
        delete_empty_objects: bool
    ) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable
            .commit)(
                self.db.state,
                block_number,
                delete_empty_objects,
                &mut error_msg as *mut UnmanagedVector
            )
            .into();

        let default = || {
            format!("Failed to commit changes to the db: {}", String::from_utf8_lossy(key))
        };
        unsafe {
            go_error.into_result(error_msg, default)?;
        }
        Ok(())
    }
}
