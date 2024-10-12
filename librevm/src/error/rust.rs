use errno::{set_errno, Errno};
use revm_primitives::{EVMError, InvalidTransaction};

use crate::memory::UnmanagedVector;

use super::BackendError;
#[repr(i32)]
pub enum ErrnoValue {
    Success = 0,
    Other = 1,
}
pub fn set_error(
    err: EVMError<BackendError, InvalidTransaction>,
    error_msg: Option<&mut UnmanagedVector>,
) {
    if let Some(error_msg) = error_msg {
        let msg: Vec<u8> = err.to_string().into();
        *error_msg = UnmanagedVector::new(Some(msg));
    } else {
        // The caller provided a nil pointer for the error message.
        // That's not nice but we can live with it.
    }

    set_errno(Errno(ErrnoValue::Other as i32));
}
