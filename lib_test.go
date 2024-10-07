package revm_api_test

import (
	"math/big"
	"testing"

	revm "github.com/rethmint/revm-api"
	api "github.com/rethmint/revm-api/api"
	"github.com/rethmint/revm-api/contracts/Call"
	"github.com/rethmint/revm-api/contracts/Counter"
	types "github.com/rethmint/revm-api/types"
)

func setupTest(t *testing.T) (revm.VM, *api.MockKVStore, types.AccountAddress) {
	vm, kvStore := startVM(t)
	caller, _ := types.NewAccountAddress("0xe100713fc15400d1e94096a545879e7c647001e0")
	faucet(kvStore, caller, big.NewInt(1000000000))

	return vm, kvStore, caller
}

func suiteTest(t *testing.T, binString string, abiString string, method []string) {
	vm, kvStore, caller := setupTest(t)

	txData := extractTxData(t, binString)
	tx := defaultTx(caller, CreateTransactTo, 0)
	block := defaultBlock()

	res, _ := api.ExecuteTx(vm.Inner, kvStore, tx, block, txData)
	t.Log("Deploy res: \n", res)

	deployedAddr, _ := types.NewAccountAddress("0xec30481c768e48d34ea8fc2bebcfeaddeba6bfa4")

	abi := parseABI(t, abiString)
	callData := extractCallData(t, abi, method[0])

	tx2 := defaultTx(caller, deployedAddr, 1)
	block2 := defaultBlock()

	res2, _ := api.ExecuteTx(vm.Inner, kvStore, tx2, block2, callData)
	t.Log("Call res: \n", res2)
}

func TestCall(t *testing.T) {
	suiteTest(t, Call.CallBin, Call.CallABI, []string{"reflect"})
}

func TestCounter(t *testing.T) {
	suiteTest(t, Counter.CounterBin, Counter.CounterABI, []string{"increase"})
}
