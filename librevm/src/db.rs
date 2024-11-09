use crate::memory::{U8SliceView, UnmanagedVector};

//use crate::{ iterator::GoIter, memory::{ U8SliceView, UnmanagedVector } };
//
// this represents something passed in from the caller side of FFI
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct db_t {
    _private: [u8; 0],
}

// These functions should return GoError but because we don't trust them here, we treat the return value as i32
// and then check it when converting to GoError manually
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct Db_vtable {
    pub read_db: extern "C" fn(
        *mut db_t,
        U8SliceView,
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector, // error message output
    ) -> i32,
    pub write_db: extern "C" fn(
        *mut db_t,
        U8SliceView,
        U8SliceView,
        *mut UnmanagedVector, // error message output
    ) -> i32,
    pub remove_db: extern "C" fn(
        *mut db_t,
        U8SliceView,
        *mut UnmanagedVector, // error message output
    ) -> i32,
}

#[repr(C)]
pub struct Db {
    pub state: *mut db_t,
    pub vtable: Db_vtable,
}

impl Default for Db {
    fn default() -> Self {
        // Initialize a null pointer for the state field (no state by default)
        let state = std::ptr::null_mut::<db_t>();

        // Initialize the vtable with default no-op functions
        let vtable = Db_vtable {
            read_db: default_read_db,
            write_db: default_write_db,
            remove_db: default_remove_db,
        };

        Db { state, vtable }
    }
}

extern "C" fn default_read_db(
    _db: *mut db_t,
    _key: U8SliceView,
    _result: *mut UnmanagedVector,
    _errmsg: *mut UnmanagedVector,
) -> i32 {
    println!("Default read_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_write_db(
    _db: *mut db_t,
    _key: U8SliceView,
    _value: U8SliceView,
    _errmsg: *mut UnmanagedVector,
) -> i32 {
    println!("Default write_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_remove_db(
    _db: *mut db_t,
    _key: U8SliceView,
    _errmsg: *mut UnmanagedVector,
) -> i32 {
    println!("Default remove_db called");
    -1 // Indicating an error or default behavior
}
