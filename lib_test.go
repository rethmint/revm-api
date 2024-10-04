package revm_api_test

import (
	"encoding/binary"
	"encoding/hex"
	"math/rand"
	"testing"

	revm "github.com/rethmint/revm-api"
	api "github.com/rethmint/revm-api/api"
	env "github.com/rethmint/revm-api/types"
	"github.com/stretchr/testify/require"
)

func generateRandomHash() [32]byte {
	bz := make([]byte, 0, 32)
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())

	var resBz [32]byte
	copy(resBz[:], bz)

	return resBz
}

func startVM(t *testing.T) (revm.VM, *api.MockKVStore) {
	kvStore := api.NewMockKVStore()
	vm := revm.NewVM()
	t.Cleanup(func() {
		vm.Destroy()
	})
	return vm, kvStore
}

func Test_ERC20_e2e(t *testing.T) {
	vm, kvStore := startVM(t)
	// @winterjihwan
	// now, you doesn't have to consider gas caller have
	// token contract create
	// token mint
	// token transfer
	// token balance query
	block := env.Block{}
	tx := env.Transaction{}
	txData, err := hex.DecodeString(txStr)
	require.NoError(t, err)
	data := []byte("test data")

	result, err := vm.ExecuteTx(kvStore, block, tx, data)
	if err != nil {
		t.Fatalf("ExecuteTx failed: %v", err)
	}

	if result == nil {
		t.Fatalf("Expected non-nil result")
	}
}
