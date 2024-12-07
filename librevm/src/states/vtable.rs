use crate::memory::{ U8SliceView, UnmanagedVector };

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
// https://github.com/ethereum/go-ethereum/blob/08e6bdb550712503873fb2a138b30132cc36c481/core/vm/interface.go#L32
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct Db_vtable {
    /// Retrieves the storage root for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the storage root is being retrieved.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the result will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub get_storage_root: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Retrieves the code for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the code is being retrieved.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the result will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub get_code: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Retrieves the code hash for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the code hash is being retrieved.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the result will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub get_code_hash: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Retrieves the state for a given address and slot hash.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the state is being retrieved.
    /// - `U8SliceView`: The slot hash for which the state is being retrieved.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the result will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub get_state: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        U8SliceView, // slot hash
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Retrieves the balance for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the balance is being retrieved.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the result will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub get_balance: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Retrieves the nonce for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the nonce is being retrieved.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the result will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub get_nonce: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Adds balance to a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address to which the balance is being added.
    /// - `U8SliceView`: The amount of balance to add.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub add_balance: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        U8SliceView, // amount
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Subtracts balance from a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address from which the balance is being subtracted.
    /// - `U8SliceView`: The amount of balance to subtract.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub sub_balance: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        U8SliceView, // amount
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Sets the balance for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the balance is being set.
    /// - `U8SliceView`: The balance to set.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub set_balance: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        U8SliceView, // balance
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Sets the nonce for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the nonce is being set.
    /// - `u64`: The nonce to set.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub set_nonce: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        u64, // nonce
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Sets the code for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the code is being set.
    /// - `U8SliceView`: The code to set.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub set_code: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        U8SliceView, // code
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Sets the state for a given address and slot hash.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the state is being set.
    /// - `U8SliceView`: The slot hash for which the state is being set.
    /// - `U8SliceView`: The value to set.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub set_state: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        U8SliceView, // slot hash
        U8SliceView, // value
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Sets the storage for a given address.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address for which the storage is being set.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where the storage input will be stored.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub set_storage: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector, // storage input
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Marks the given account as selfdestructed.
    ///
    /// This clears the account balance. The account's state object is still available until the state is committed,
    /// getStateObject will return a non-nil account after SelfDestruct.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `U8SliceView`: The address of the account to selfdestruct.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub self_destruct: extern "C" fn(
        *mut db_t,
        U8SliceView, // address
        *mut UnmanagedVector // error message output
    ) -> i32,

    /// Commits the state mutations into the configured data stores.
    ///
    /// # Parameters
    /// - `db_t`: A mutable pointer to the database.
    /// - `u64`: The block number associated with the state transition.
    /// - `bool`: Flag indicating whether to delete empty objects.
    /// - `UnmanagedVector`: A mutable pointer to an unmanaged vector where any error message will be stored.
    ///
    /// # Returns
    /// - `i32`: Status code indicating success or failure.
    pub commit: extern "C" fn(
        *mut db_t,
        u64, // block number
        bool, // delete empty objects
        *mut UnmanagedVector // error message output
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
            get_storage_root: default_read_db,
            get_code: default_read_db,
            get_code_hash: default_read_db,
            get_state: default_read_db2,
            get_balance: default_read_db,
            get_nonce: default_read_db,
            add_balance: default_write_db,
            sub_balance: default_write_db,
            set_balance: default_write_db,
            set_nonce: default_write_db2,
            set_code: default_write_db,
            set_state: default_write_db3,
            set_storage: default_write_db4,
            commit: default_commit,
        };

        Db { state, vtable }
    }
}

extern "C" fn default_read_db(
    _db: *mut db_t,
    _key: U8SliceView,
    _result: *mut UnmanagedVector,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default read_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_read_db2(
    _db: *mut db_t,
    _key: U8SliceView,
    _key2: U8SliceView,
    _result: *mut UnmanagedVector,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default read_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_write_db(
    _db: *mut db_t,
    _key: U8SliceView,
    _value: U8SliceView,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default write_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_write_db2(
    _db: *mut db_t,
    _key: U8SliceView,
    _value: u64,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default write_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_write_db3(
    _db: *mut db_t,
    _key: U8SliceView,
    _key2: U8SliceView,
    _value: U8SliceView,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default write_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_write_db4(
    _db: *mut db_t,
    _key: U8SliceView,
    _value: UnmanagedVector,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default write_db called");
    -1 // Indicating an error or default behavior
}

extern "C" fn default_write_db5(
    _db: *mut db_t,
    _key: u64,
    _key2: bool,
    _errmsg: *mut UnmanagedVector
) -> i32 {
    println!("Default write_db called");
    -1 // Indicating an error or default behavior
}
