use crate::memory::{U8SliceView, UnmanagedVector};

use anyhow::anyhow;

// this represents something passed in from the caller side of FFI
// in this case a struct with go function pointers
#[repr(C)]
pub struct api_t {
    _private: [u8; 0],
}

// These functions should return GoError but because we don't trust them here, we treat the return value as i32
// and then check it when converting to GoError manually
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi_vtable {
    pub query: extern "C" fn(
        *const api_t,
        U8SliceView, // request
        u64,
        *mut UnmanagedVector, // response
        *mut u64,
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub get_account_info: extern "C" fn(
        *const api_t,
        U8SliceView,          // addr
        *mut bool,            // found
        *mut u64,             // account_number
        *mut u64,             // sequence
        *mut u8,              // account_type
        *mut bool,            // is_blocked
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub amount_to_share: extern "C" fn(
        *const api_t,
        U8SliceView,          // validator
        U8SliceView,          // metadata
        u64,                  // amount
        *mut UnmanagedVector, // share
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub share_to_amount: extern "C" fn(
        *const api_t,
        U8SliceView,          // validator
        U8SliceView,          // metadata
        U8SliceView,          // share
        *mut u64,             // amount
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub unbond_timestamp: extern "C" fn(
        *const api_t,
        *mut u64,             // unbond_timestamp
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub get_price: extern "C" fn(
        *const api_t,
        U8SliceView,          // pair_id
        *mut UnmanagedVector, // price
        *mut u64,             // updated_at
        *mut u64,             // decimals
        *mut UnmanagedVector, // error_msg
    ) -> i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi {
    pub state: *const api_t,
    pub vtable: GoApi_vtable,
}

// We must declare that these are safe to Send, to use in wasm.
// The known go caller passes in immutable function pointers, but this is indeed
// unsafe for possible other callers.
//
// see: https://stackoverflow.com/questions/50258359/can-a-struct-containing-a-raw-pointer-implement-send-and-be-ffi-safe
unsafe impl Send for GoApi {}
