package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"runtime"

	types "github.com/rethmint/revm-api/types"
)

type VM struct {
	ptr *C.evm_t
}

// ReleaseVM call ffi(`release_vm`) to release vm instance
func ReleaseVM(vm VM) {
	C.release_vm(vm.ptr)
}

// InitVM call ffi(`init_vm`) to initialize vm instance
func InitVM() VM {
	return VM{
		ptr: C.init_vm(),
	}
}

// ExecuteTx call ffi(`execute_tx`) to execute
func ExecuteTx(
	vm VM,
	store KVStore,
	tx types.Transaction,
	block types.Block,
) (types.ExecutionResult, error) {
	var err error

	dbState := buildDBState(store)
	db := buildDB(&dbState)
	// tx -> byte -> ByteSliceView
	// block -> byte -> ByteSliceView
	blockBytesSliceView := makeView(block.ToJsonStringBytes())
	defer runtime.KeepAlive(blockBytesSliceView)
	txByteSliceView := makeView(tx.ToJsonStringBytes())
	defer runtime.KeepAlive(txByteSliceView)
	// TODO: handle error msg
	// errmsg := uninitializedUnmanagedVector()
	res, err := C.execute_tx(vm.ptr, db, blockBytesSliceView, txByteSliceView)
	// if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
	// 	return nil, errorWithMessage(err, errmsg)
	// }
	return copyAndDestroyUnmanagedVector(res), err
}

func Query(
	vm VM,
	store KVStore,
	tx types.Transaction,
	block types.Block,
) (types.ExecutionResult, error) {
	var err error

	dbState := buildDBState(store)
	db := buildDB(&dbState)
	// tx -> byte -> ByteSliceView
	// block -> byte -> ByteSliceView
	blockBytesSliceView := makeView(block.ToJsonStringBytes())
	defer runtime.KeepAlive(blockBytesSliceView)
	txByteSliceView := makeView(tx.ToJsonStringBytes())
	defer runtime.KeepAlive(txByteSliceView)
	// TODO: handle error msg
	// errmsg := uninitializedUnmanagedVector()
	res, err := C.query(vm.ptr, db, blockBytesSliceView, txByteSliceView)
	// if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
	// 	return nil, errorWithMessage(err, errmsg)
	// }
	return copyAndDestroyUnmanagedVector(res), err
}
