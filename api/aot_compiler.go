package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

type Compiler struct {
	ptr *C.compiler_t
}

func ReleaseCompiler(compiler Compiler) {
	C.release_compiler(compiler.ptr)
}

func InitCompiler() Compiler {
	return Compiler{
		ptr: C.init_compiler(),
	}
}

func StartRoutine(compiler Compiler, store KVStore) {
	dbState := buildDBState(store)
	db := buildDB(&dbState)
	C.start_routine(compiler.ptr, db)
}
