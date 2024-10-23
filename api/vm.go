package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"runtime"
	"syscall"

	types "github.com/rethmint/revm-api/types/go"
)

type VM struct {
	ptr *C.evm_t
}

// ReleaseVM call ffi(`release_vm`) to release vm instance
func ReleaseVM(vm VM) {
	C.release_vm(vm.ptr)
}

// InitVM call ffi(`init_vm`) to initialize vm instance
func InitVM(SPEC_ID uint8) VM {
	return VM{
		ptr: C.init_vm(cu8(SPEC_ID)),
	}
}

// ExecuteTx call ffi(`execute_tx`) to execute
func ExecuteTx(
	vm VM,
	store KVStore,
	block []byte,
	tx []byte,
) (types.ExecutionResult, error) {
	var err error

	dbState := buildDBState(store)
	db := buildDB(&dbState)
	blockBytesSliceView := makeView(block)
	defer runtime.KeepAlive(blockBytesSliceView)
	txByteSliceView := makeView(tx)
	defer runtime.KeepAlive(txByteSliceView)

	errmsg := uninitializedUnmanagedVector()
	res, err := C.execute_tx(vm.ptr, db, blockBytesSliceView, txByteSliceView, &errmsg)
	if err != nil && err.(syscall.Errno) != C.Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func Query(
	vm VM,
	store KVStore,
	block []byte,
	tx []byte,
) (types.ExecutionResult, error) {
	var err error

	dbState := buildDBState(store)
	db := buildDB(&dbState)
	blockBytesSliceView := makeView(block)
	defer runtime.KeepAlive(blockBytesSliceView)
	txByteSliceView := makeView(tx)
	defer runtime.KeepAlive(txByteSliceView)

	errmsg := uninitializedUnmanagedVector()
	res, err := C.query(vm.ptr, db, blockBytesSliceView, txByteSliceView, &errmsg)
	if err != nil && err.(syscall.Errno) != C.Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}
