package revm_api_test

import (
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"math/big"
	"math/rand"
	"strings"
	"testing"

	"github.com/ethereum/go-ethereum/accounts/abi"
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

func faucet(kvStore *api.MockKVStore, address types.AccountAddress, amount *big.Int) {
	accountKey := AddressToAccountAddressKey(address)

	var accountInfoBytes []byte
	if accountInfoBytes = kvStore.Get(accountKey); accountInfoBytes == nil {
		accountInfoBytes, _ = kvStore.CreateEOA(accountKey)
	}

	accountInfo, _ := types.AccountInfoFromBytes(accountInfoBytes)
	accountInfo.Balance = accountInfo.Balance.Add(accountInfo.Balance, amount)

	kvStore.Set(accountKey, accountInfo.ToBytes())
}

func TestCallEOA(t *testing.T) {
	vm, kvStore := startVM(t)
	caller, _ := types.NewAccountAddress("0xe100713fc15400d1e94096a545879e7c647001e0")

	faucet(kvStore, caller, big.NewInt(1000000000))

	txStr := Call.CallBin

	if txStr[:2] == "0x" {
		txStr = txStr[2:]
	}

	txData, err := hex.DecodeString(txStr)
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

	res, _ := api.ExecuteTx(vm.Inner, kvStore, block, tx, txData)
	fmt.Println("Res: \n", res)

	deployedAddress, _ := types.NewAccountAddress("0xec30481c768e48d34ea8fc2bebcfeaddeba6bfa4")

	parsedABI, err := abi.JSON(strings.NewReader(Call.CallABI))
	if err != nil {
		t.Fatalf("Failed to parse ABI: %v", err)
	}

	txData2, err := parsedABI.Pack("reflect")
	if err != nil {
		t.Fatalf("Failed to pack method call: %v", err)
	}

	tx2 := types.Transaction{
		Caller:         caller,
		GasLimit:       0xf4240,
		GasPrice:       big.NewInt(1000),
		TransactTo:     deployedAddress,
		Value:          big.NewInt(0),
		Nonce:          1,
		ChainId:        1,
		GasPriorityFee: big.NewInt(1000),
	}

	block2 := types.Block{
		Number:    big.NewInt(1),
		Coinbase:  types.ZeroAddress(),
		Timestamp: big.NewInt(0),
		GasLimit:  big.NewInt(10000000),
		Basefee:   big.NewInt(0),
	}

	res2, _ := api.ExecuteTx(vm.Inner, kvStore, block2, tx2, txData2)
	fmt.Println("Res2: \n", res2)

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
