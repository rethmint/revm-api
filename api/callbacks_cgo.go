package api

/*
#include "bindings.h"
#include <stdio.h>

// imports (db)
GoError cGet(db_t *ptr, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut);
GoError cSet(db_t *ptr, U8SliceView key, U8SliceView val, UnmanagedVector *errOut);
GoError cDelete(db_t *ptr, U8SliceView key, UnmanagedVector *errOut);

// Gateway functions (db)
GoError cGet_cgo(db_t *ptr, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut) {
	return cGet(ptr, key, val, errOut);
}
GoError cSet_cgo(db_t *ptr, U8SliceView key, U8SliceView val, UnmanagedVector *errOut) {
	return cSet(ptr, key, val, errOut);
}
GoError cDelete_cgo(db_t *ptr, U8SliceView key, UnmanagedVector *errOut) {
	return cDelete(ptr, key, errOut);
}
*/
import "C"

// We need these gateway functions to allow calling back to a go function from the c code.
// At least I didn't discover a cleaner way.
// Also, this needs to be in a different file than `callbacks.go`, as we cannot create functions
// in the same file that has //export directives. Only import header types
