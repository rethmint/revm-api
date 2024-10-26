package api

import (
	"encoding/hex"
	"math/big"

	dbm "github.com/cosmos/cosmos-db"
	types "github.com/rethmint/revm-api/types/go"
)

const KECCAK_EMPTY = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"

/*** Mock KVStore ****/

type MockKVStore struct {
	db *dbm.MemDB
}

func NewMockKVStore() *MockKVStore {
	return &MockKVStore{
		db: dbm.NewMemDB(),
	}
}

func (k *MockKVStore) CreateEOA(accountKey []byte) ([]byte, error) {
	var codeHashBytes [32]byte
	codeHash, err := hex.DecodeString(KECCAK_EMPTY)
	if err != nil {
		return []byte{}, err
	}

	copy(codeHashBytes[:], codeHash)

	accountInfo := types.AccountInfo{
		Balance:  big.NewInt(0),
		Nonce:    0,
		CodeHash: codeHashBytes,
	}

	k.Set(accountKey, accountInfo.ToBytes())

	return accountInfo.ToBytes(), nil
}

// Get wraps the underlying DB's Get method panicing on error.
func (l MockKVStore) Get(key []byte) []byte {
	v, err := l.db.Get(key)
	if err != nil {
		panic(err)
	}

	return v
}

// Set wraps the underlying DB's Set method panicing on error.
func (l MockKVStore) Set(key, value []byte) {
	if err := l.db.Set(key, value); err != nil {
		panic(err)
	}
}

// Delete wraps the underlying DB's Delete method panicing on error.
func (l MockKVStore) Delete(key []byte) {
	if err := l.db.Delete(key); err != nil {
		panic(err)
	}
}

// Iterator wraps the underlying DB's Iterator method panicing on error.
func (l MockKVStore) Iterator(start, end []byte) dbm.Iterator {
	iter, err := l.db.Iterator(start, end)
	if err != nil {
		panic(err)
	}

	return iter
}

// ReverseIterator wraps the underlying DB's ReverseIterator method panicing on error.
func (l MockKVStore) ReverseIterator(start, end []byte) dbm.Iterator {
	iter, err := l.db.ReverseIterator(start, end)
	if err != nil {
		panic(err)
	}

	return iter
}

var _ KVStore = (*MockKVStore)(nil)
