package revm_api_test

import (
	"encoding/hex"
	"math/big"
	"strings"
	"testing"

	"github.com/ethereum/go-ethereum/accounts/abi"
	revm "github.com/rethmint/revm-api"
	api "github.com/rethmint/revm-api/api"
	"github.com/rethmint/revm-api/contracts/Call"
	types "github.com/rethmint/revm-api/types/go"
)

func startVM(t *testing.T) (revm.VM, *api.MockKVStore) {
	kvStore := api.NewMockKVStore()
	vm := revm.NewVM()
	t.Cleanup(func() {
		vm.Destroy()
	})
	return vm, kvStore
}

var AccountPrefix = [1]byte{0x01}
var CreateTransactTo = [20]uint8{0}

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

func extractTxData(t *testing.T, txStr string) []byte {
	if txStr[:2] == "0x" {
		txStr = txStr[2:]
	}

	txData, err := hex.DecodeString(txStr)
	if err != nil {
		t.Fatal(err)
	}

	return txData
}

func extractCallData(t *testing.T, abi abi.ABI, method string) []byte {
	callData, err := abi.Pack(method)
	if err != nil {
		t.Fatalf("Failed to pack method call: %v", err)
	}
	return callData
}

func defaultTx(caller types.AccountAddress, transactTo [20]byte, txData []byte, nonce uint64) types.Transaction {
	return types.Transaction{
		Caller:         caller,
		GasLimit:       0xf4240,
		GasPrice:       types.NewU256(big.NewInt(10000)),
		TransactTo:     transactTo,
		Value:          types.NewU256(big.NewInt(0)),
		Data:           txData,
		Nonce:          nonce,
		ChainId:        1,
		GasPriorityFee: types.NewU256(big.NewInt(0)),
	}
}

func defaultBlock() types.Block {
	return types.Block{
		Number:    types.NewU256(big.NewInt(1)),
		Coinbase:  types.ZeroAddress(),
		Timestamp: types.NewU256(big.NewInt(1000000)),
		GasLimit:  types.NewU256(big.NewInt(10000000)),
		Basefee:   types.NewU256(big.NewInt(0)),
	}

}

func parseABI(t *testing.T, dest_abi string) abi.ABI {
	parsedABI, err := abi.JSON(strings.NewReader(dest_abi))
	if err != nil {
		t.Fatalf("Failed to parse ABI: %v", err)
	}

	return parsedABI
}
func setupTest(t *testing.T) (revm.VM, *api.MockKVStore, types.AccountAddress) {
	vm, kvStore := startVM(t)
	caller, _ := types.NewAccountAddress("0xe100713fc15400d1e94096a545879e7c647001e0")
	faucet(kvStore, caller, big.NewInt(1000000000000))

	return vm, kvStore, caller
}

func suiteTest(t *testing.T, binString string, abiString string, method []string) {
	vm, kvStore, caller := setupTest(t)

	txData := extractTxData(t, binString)

	tx := defaultTx(caller, CreateTransactTo, txData, 0)
	block := defaultBlock()

	res, _ := vm.ExecuteTx(kvStore, block, tx)

	successRes, _ := res.(types.Success)

	deployedAddr := successRes.Output.DeployedAddress

	abi := parseABI(t, abiString)
	res = extractCallData(t, abi, method[0])

	callData := extractCallData(t, abi, method[0])

	tx2 := defaultTx(caller, deployedAddr, callData, 1)
	block2 := defaultBlock()

	callRes, _ := vm.ExecuteTx(kvStore, block2, tx2)

	_, ok := callRes.(types.Success)
	if ok == false {
		t.Fatal("Call res not success")
	}
}

func TestCall(t *testing.T) {
	suiteTest(t, Call.CallBin, Call.CallABI, []string{"reflect"})
}
