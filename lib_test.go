package revm_api_test

import (
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	revm "github.com/rethmint/revm-api"
	fibca "github.com/rethmint/revm-api/contracts/Fibonacci"
	testutils "github.com/rethmint/revm-api/testutils"
	types "github.com/rethmint/revm-api/types/go"
	"github.com/stretchr/testify/require"
)

const CANCUN uint8 = 17

func setupTest(t *testing.T, aot bool) (revm.VM, *testutils.MockKVStore, common.Address) {
	kvStore := testutils.NewMockKVStore()

	var compiler revm.Compiler
	var vm revm.VM

	if aot {
		compiler = revm.NewCompiler(1000, 0)
		vm = revm.NewAotVM(CANCUN, compiler)

		go func() {
			compiler.Start()
		}()
	} else {
		vm = revm.NewVM(CANCUN)
	}

	t.Cleanup(func() {
		time.Sleep(3 * time.Second)
		vm.Destroy()
		if aot {
			compiler.Destroy()
		}
	})

	caller := common.HexToAddress("0xe100713fc15400d1e94096a545879e7c647001e0")
	testutils.Faucet(kvStore, caller, big.NewInt(1000000000000))

	return vm, kvStore, caller
}

func Test_e2e_non_aot(t *testing.T) {
	aot := false
	Fib(t, aot)
}

func Test_e2e_aot(t *testing.T) {
	aot := true
	Fib(t, aot)
}

func Fib(t *testing.T, aot bool) {
	vm, kvStore, caller := setupTest(t, aot)

	txData, err := hexutil.Decode(fibca.FibonacciBin)
	require.NoError(t, err)
	createTx := testutils.MockTx(caller, common.Address{}, txData, 0)
	block := testutils.MockBlock(1)
	res, err := vm.ExecuteTx(kvStore, block.ToSerialized(), createTx.ToSerialized())
	require.NoError(t, err)
	result, err := res.ProcessExecutionResult()
	require.NoError(t, err)
	createRes, ok := result.(types.Success)
	require.True(t, ok)
	deployedAddr := createRes.Output.DeployedAddress

	abi, err := fibca.FibonacciMetaData.GetAbi()
	require.NoError(t, err)

	for i := 0; i < 5; i++ {
		testInput, err := abi.Pack("fibonacci", big.NewInt(25))
		require.NoError(t, err)

		start := time.Now()

		testTx := testutils.MockTx(caller, deployedAddr, testInput, uint64(i+1))
		block = testutils.MockBlock(int64(2 + i))
		res, err = vm.ExecuteTx(kvStore, block.ToSerialized(), testTx.ToSerialized())
		require.NoError(t, err)

		result, err = res.ProcessExecutionResult()
		require.NoError(t, err)

		fibRes, ok := result.(types.Success)
		require.True(t, ok)

		require.Equal(t, types.Success{
			Reason:      "Return",
			GasUsed:     0x2bff4bd,
			GasRefunded: 0x0,
			Logs:        []types.Log{},
			Output: types.Output{
				DeployedAddress: common.Address{},
				Output: []uint8{
					0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x25, 0x11,
				},
			},
		}, fibRes)

		elapsed := time.Since(start)
		t.Logf("Test Aot: Call %d execution time: %v", i+1, elapsed)

		time.Sleep(1 * time.Second)
	}
}
