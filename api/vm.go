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
func InitVM(SPEC_ID uint8) VM {
	return VM{
		evm_ptr: C.init_vm(cu8(SPEC_ID)),
		aot:     false,
	}
}

// InitAotVM call ffi(`init_aot_vm`) to initialize vm instance
func InitAotVM(SPEC_ID uint8, compiler Compiler) VM {
	return VM{
		evm_ptr: C.init_aot_vm(cu8(SPEC_ID), compiler.ptr),
		aot:     true,
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
		// ignore the opereation times out error
		errno, ok := err.(syscall.Errno)
		if ok && errno == syscall.ETIMEDOUT {
			return copyAndDestroyUnmanagedVector(res), nil
		}
		// get_function checks if file exists, enoent: file not found
		if ok && errno == syscall.ENOENT {
			return copyAndDestroyUnmanagedVector(res), nil
		}
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
		// ignore the operation timed out error
		errno, ok := err.(syscall.Errno)
		if ok && errno == syscall.ETIMEDOUT {
			return copyAndDestroyUnmanagedVector(res), nil
		}
		// get_function checks if file exists, enoent: file not found
		if ok && errno == syscall.ENOENT {
			return copyAndDestroyUnmanagedVector(res), nil
		}
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

func InitCompiler(threshold uint64) Compiler {
	return Compiler{
		ptr: C.init_compiler(C.uint64_t(threshold)),
	}
}
