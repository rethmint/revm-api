package api_test

import (
	"testing"

	"github.com/rethmint/revm-api/api"
)

func TestRevmKVStore(t *testing.T) {
	store := api.NewRevmKVStore()

	key1 := []byte("key1")
	value1 := []byte("value1")
	store.Set(key1, value1)

	if got := store.Get(key1); string(got) != string(value1) {
		t.Errorf("Get(%s) = %s; want %s", key1, got, value1)
	}

	key2 := []byte("key2")
	if got := store.Get(key2); got != nil {
		t.Errorf("Get(%s) = %s; want nil", key2, got)
	}

	value2 := []byte("value2")
	store.Set(key2, value2)

	if got := store.Get(key2); string(got) != string(value2) {
		t.Errorf("Get(%s) = %s; want %s", key2, got, value2)
	}

	store.Delete(key1)

	if got := store.Get(key1); got != nil {
		t.Errorf("Get(%s) after Delete = %s; want nil", key1, got)
	}
}

func TestRevmKVStore_Delete_NonExistingKey(t *testing.T) {
	store := api.NewRevmKVStore()
	key := []byte("nonexistent_key")

	// Delete a non-existing key should not cause an error
	store.Delete(key)

	// Ensure no panic occurs, and Get returns nil
	if got := store.Get(key); got != nil {
		t.Errorf("Get(%s) after Delete = %s; want nil", key, got)
	}
}
