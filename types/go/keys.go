package types

// KV Store Prefix
// ACCOUNT_PREFIX(B1) + {address(B20)} => ACCOUNT INFO {balance(B64)(0) | nonce(B256)(1) | code_hash(B256)(2)}
// CODE_PREFIX(B1) + {code_hash(B32)} => vm bytecode
// STORAGE_PREFIX(B1) + {address(B20)} + {index(B32)} => [32]byte(value)
// BLOCK_PREFIX(B1) + block_num(B8) => block_hash
var (
	VMStorePrefix = []byte{0x20}
)

var (
	AccountPrefix = []byte{0x01}
	CodePrefix    = []byte{0x02}
	StoragePrefix = []byte{0x03}
	BlockPrefix   = []byte{0x04}
)
