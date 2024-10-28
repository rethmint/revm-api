package testutils

import (
	"encoding/hex"
	"math/big"

	dbm "github.com/cosmos/cosmos-db"
)

const KECCAK_EMPTY = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"

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

	accountInfo := AccountInfo{
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

// KVStore defines the interface for key-value store operations.
type KVStore interface {
	Get(key []byte) []byte
	Set(key, value []byte)
	Delete(key []byte)
	Iterator(start, end []byte) dbm.Iterator
	ReverseIterator(start, end []byte) dbm.Iterator
}

var _ KVStore = (*MockKVStore)(nil)
