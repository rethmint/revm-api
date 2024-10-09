package revm_api_test

import (
	"fmt"
	"math/big"
	"testing"

	revm "github.com/rethmint/revm-api"
	api "github.com/rethmint/revm-api/api"
	"github.com/rethmint/revm-api/contracts/Call"
	"github.com/rethmint/revm-api/contracts/Counter"
	types "github.com/rethmint/revm-api/types/go"
)

func setupTest(t *testing.T) (revm.VM, *api.MockKVStore, types.AccountAddress) {
	vm, kvStore := startVM(t)
	caller, _ := types.NewAccountAddress("0xe100713fc15400d1e94096a545879e7c647001e0")
	faucet(kvStore, caller, big.NewInt(1000000000000))

	return vm, kvStore, caller
}

func suiteTest(t *testing.T, binString string, abiString string, method []string) {
	vm, kvStore, caller := setupTest(t)

	// txData := extractTxData(t, binString)
	tx := defaultTx(caller, CreateTransactTo, 0)
	block := defaultBlock()

	res, _ := vm.ExecuteTx(kvStore, block, tx)
	t.Log("Deploy Res: \n", res)

	deployedAddr, _ := types.NewAccountAddress("0xec30481c768e48d34ea8fc2bebcfeaddeba6bfa4")

	// abi := parseABI(t, abiString)
	// callData := extractCallData(t, abi, method[0])

	return vm, kvStore, caller
}

func suiteTestWithArgs(t *testing.T, txData []byte, abiString string, method []string) {
	vm, kvStore, caller := setupTest(t)

	tx := defaultTx(caller, CreateTransactTo, txData, 0)
	block := defaultBlock()
	res, _ := vm.ExecuteTx(kvStore, block, tx)
	fmt.Println("Res: \n", res)

	deployedAddr, _ := types.NewAccountAddress("0xec30481c768e48d34ea8fc2bebcfeaddeba6bfa4")

	// abi := parseABI(t, abiString)
	// callData := extractCallData(t, abi, method[0])

	// tx2 := defaultTx(caller, deployedAddr, callData, 1)
	// block2 := defaultBlock()

	// res2, _ := vm.ExecuteTx(kvStore, block2, tx2)
	// t.Log("Call res: \n", res2)
}

func TestCall(t *testing.T) {
	suiteTest(t, Call.CallBin, Call.CallABI, []string{"reflect"})
}

func TestCounter(t *testing.T) {
	suiteTest(t, Counter.CounterBin, Counter.CounterABI, []string{"increase"})
}
