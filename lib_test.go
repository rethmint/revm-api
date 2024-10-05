package revm_api_test

import (
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"math/big"
	"math/rand"
	"testing"

	revm "github.com/rethmint/revm-api"
	api "github.com/rethmint/revm-api/api"
	"github.com/rethmint/revm-api/contracts/Call"

	// call "github.com/rethmint/revm-api/contracts/call"
	// env "github.com/rethmint/revm-api/types"
	types "github.com/rethmint/revm-api/types"
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

var AccountPrefix = [1]byte{0x01}

type AccountAddressKey []byte

func AddressToAccountAddressKey(address types.AccountAddress) AccountAddressKey {
	result := make([]byte, 1)
	copy(result[:], AccountPrefix[:])

	result = append(result, address[:]...)

	return result
}

func createEOA(t *testing.T, address types.AccountAddress) {

}

func faucet(t *testing.T, kvStore *api.MockKVStore, address types.AccountAddress, amount *big.Int) {
	accountKey := AddressToAccountAddressKey(address)
	balanceBytes := amount.Bytes()

	kvStore.Set(accountKey, balanceBytes)
}

func TestCallEOA(t *testing.T) {
	vm, kvStore := startVM(t)
	caller, _ := types.NewAccountAddress("0xe100713fc15400d1e94096a545879e7c647001e0")

	faucet(t, kvStore, caller, big.NewInt(1000000000))

	txStr := Call.CallBin

	if txStr[:2] == "0x" {
		txStr = txStr[2:]
	}

	byteArray, err := hex.DecodeString(txStr)
	if err != nil {
		t.Fatal(err)
	}

	tx := types.Transaction{
		Caller:         caller,
		GasLimit:       0xf4240,
		GasPrice:       big.NewInt(1000),
		TransactTo:     [20]uint8{0},
		Value:          big.NewInt(0),
		Nonce:          0,
		ChainId:        1,
		GasPriorityFee: big.NewInt(1000),
	}

	block := types.Block{
		Number:    big.NewInt(1),
		Coinbase:  types.ZeroAddress(),
		Timestamp: big.NewInt(0),
		GasLimit:  big.NewInt(10000000),
		Basefee:   big.NewInt(0),
	}

	res, _ := api.ExecuteTx(vm.Inner, kvStore, block, tx, byteArray)
	fmt.Println("Res: \n", res)

}

func Test_ERC20_e2e(t *testing.T) {
	// vm, kvStore := startVM(t)
	// // @winterjihwan
	// // contract = ERc20
	// // now, you doesn't have to consider gas caller have
	// // token contract create
	// // token mint
	// // token transfer
	// // token balance query
	// block := env.Block{}
	// tx := env.Transaction{}
	// txData, err := hex.DecodeString(txStr)
	// require.NoError(t, err)
	// data := []byte("test data")
	//
	// result, err := vm.ExecuteTx(kvStore, block, tx, data)
	// if err != nil {
	// 	t.Fatalf("ExecuteTx failed: %v", err)
	// }
	//
	// if result == nil {
	// 	t.Fatalf("Expected non-nil result")
	// }
}
