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
	evm_ptr *C.evm_t
	aot     bool
}

// ReleaseVM call ffi(`release_vm`) to release vm instance
func ReleaseVM(vm VM) {
	C.release_vm(vm.evm_ptr, C.bool(vm.aot))
}

// InitVM call ffi(`init_vm`) to initialize vm instance
func InitVM(SPEC_ID uint8, aot bool) VM {
	return VM{
		evm_ptr: C.init_vm(cu8(SPEC_ID)),
		aot:     aot,
	}
}

// ExecuteTx call ffi(`execute_tx`) to execute
func ExecuteTx(
	vm VM,
	store KVStore,
	block *[]byte,
	tx *[]byte,
) (types.ExecutionResult, error) {
	var err error

	dbState := buildDBState(store)
	db := buildDB(&dbState)
	blockBytesSliceView := makeView(*block)
	defer runtime.KeepAlive(blockBytesSliceView)
	txByteSliceView := makeView(*tx)
	defer runtime.KeepAlive(txByteSliceView)

	errmsg := uninitializedUnmanagedVector()
	res, err := C.execute_tx(vm.evm_ptr, C.bool(vm.aot), db, blockBytesSliceView, txByteSliceView, &errmsg)
	if err != nil && err.(syscall.Errno) != C.Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func QueryTx(
	vm VM,
	store KVStore,
	block *[]byte,
	tx *[]byte,
) (types.ExecutionResult, error) {
	var err error

	dbState := buildDBState(store)
	db := buildDB(&dbState)
	blockBytesSliceView := makeView(*block)
	defer runtime.KeepAlive(blockBytesSliceView)
	txByteSliceView := makeView(*tx)
	defer runtime.KeepAlive(txByteSliceView)

	errmsg := uninitializedUnmanagedVector()
	res, err := C.query_tx(vm.evm_ptr, C.bool(vm.aot), db, blockBytesSliceView, txByteSliceView, &errmsg)
	if err != nil && err.(syscall.Errno) != C.Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

type Compiler struct {
	ptr *C.compiler_t
}

func ReleaseCompiler(compiler Compiler) {
	C.release_compiler(compiler.ptr)
}

func InitCompiler(interval uint64, threshold uint64) Compiler {
	return Compiler{
		ptr: C.init_compiler(C.uint64_t(interval), C.uint64_t(threshold)),
	}
}

func StartRoutine(compiler Compiler) {
	C.start_routine(compiler.ptr)
}
