package types

import (
	"encoding/binary"

	"github.com/ethereum/go-ethereum/common"
)

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

func CodeKey(addr common.Address, codeHash []byte) []byte {
	return append(addr.Bytes(), append(CodePrefix, codeHash...)...)
}

func BlockKey(blockNum uint64) []byte {
	return append(CodePrefix, uint64ToBytes(blockNum)...)
}

func StorageKey(addr common.Address, slot common.Hash) []byte {
	return append(append(StoragePrefix, addr.Bytes()...), slot.Bytes()...)
}

func AccountKey(addr common.Address) []byte {
	return append(AccountPrefix, addr.Bytes()...)
}

func uint64ToBytes(v uint64) []byte {
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, v)
	return bz
}
