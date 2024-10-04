package api

import (
	dbm "github.com/cosmos/cosmos-db"
)

type RevmKVStore struct {
	data map[string][]byte
}

func NewRevmKVStore() *RevmKVStore {
	return &RevmKVStore{
		data: make(map[string][]byte),
	}
}

func (store *RevmKVStore) Get(key []byte) []byte {
	return store.data[string(key)]
}

func (store *RevmKVStore) Set(key, value []byte) {
	store.data[string(key)] = value
}

func (store *RevmKVStore) Delete(key []byte) {
	delete(store.data, string(key))
}

func (store *RevmKVStore) Iterator(start, end []byte) dbm.Iterator {
	panic("Iterator not implemented")
}

func (store *RevmKVStore) ReverseIterator(start, end []byte) dbm.Iterator {
	panic("ReverseIterator not implemented")
}
