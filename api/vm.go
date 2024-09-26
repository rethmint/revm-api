package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"runtime"
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

type Transaction struct{}
type Block struct{}

// ExecuteTx call ffi(`execute_tx`) to execute
func ExecuteTx(
	vm VM,
	store KVStore,
	tx Transaction,
	block Block,
) ([]byte, error) {
	var err error

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)
	// tx -> byte -> ByteSliceView
	// block -> byte -> ByteSliceView
	e := makeView(env)
	defer runtime.KeepAlive(e)
	sendersView := makeView(senders)
	defer runtime.KeepAlive(sendersView)
	msg := makeView(message)
	defer runtime.KeepAlive(msg)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.execute_tx(vm.ptr, db, block, tx)
	// TODO: handle the error
	// if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
	// 	return nil, errorWithMessage(err, errmsg)
	// }
	// TODO result marshal
	return copyAndDestroyUnmanagedVector(res), err
}
