package revm_api_test

import (
	"fmt"
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	revm "github.com/rethmint/revm-api"
	Erc20 "github.com/rethmint/revm-api/contracts/Erc20"
	fibca "github.com/rethmint/revm-api/contracts/Fibonacci"
	testutils "github.com/rethmint/revm-api/testutils"
	types "github.com/rethmint/revm-api/types/go"
	"github.com/stretchr/testify/require"
)

type TestContract struct {
	name      string
	aot       bool
	caller    common.Address
	bin       []byte
	abi       *abi.ABI
	txdata    []byte
	calldatas []CallData
	repeat    int
}

type CallData struct {
	name     string
	calldata []byte
	expected types.Success
}

const CANCUN uint8 = 17

func setupTest(t *testing.T, aot bool, caller common.Address) (revm.VM, *testutils.MockKVStore) {
	kvStore := testutils.NewMockKVStore()

	var compiler revm.Compiler
	var vm revm.VM

	if aot {
		compiler = revm.NewCompiler(1)
		vm = revm.NewAotVM(CANCUN, compiler)
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

	testutils.Faucet(kvStore, caller, big.NewInt(1000000000000))

	return vm, kvStore
}

func Test_e2e_erc20_non_aot(t *testing.T) {
	aot := false
	Erc20CA(t, aot)
}

func Test_e2e_erc20_aot(t *testing.T) {
	aot := true
	Erc20CA(t, aot)
}

func Erc20CA(t *testing.T, aot bool) {
	caller := common.HexToAddress("0xe100713fc15400d1e94096a545879e7c647001e0")

	erc20abi, _ := Erc20.Erc20MetaData.GetAbi()
	erc20bin, _ := hexutil.Decode(Erc20.Erc20Bin)
	packedData, _ := erc20abi.Constructor.Inputs.Pack("Mock", "Mock")
	txdata := append(erc20bin, packedData...)

	mintdata, _ := erc20abi.Pack("mint", caller, big.NewInt(1000))
	mintCallData := CallData{
		name:     "mint()",
		calldata: mintdata,
		expected: types.Success{
			Reason:      "Stop",
			GasUsed:     0x11d96,
			GasRefunded: 0x0,
			Logs: []types.Log{
				{
					Address: common.HexToAddress("0xeC30481c768e48D34Ea8fc2bEbcfeAddEBA6bfA4"),
					Data: types.LogData{
						Topics: []common.Hash{
							common.HexToHash("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"),
							common.HexToHash("0x0000000000000000000000000000000000000000000000000000000000000000"),
							common.HexToHash("0x000000000000000000000000e100713fc15400d1e94096a545879e7c647001e0"),
						}, Data: []uint8{0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0xe8},
					},
				},
			}, Output: types.Output{
				DeployedAddress: common.Address{},
				Output:          []uint8{},
			},
		},
	}

	ca := TestContract{
		name:   "Erc20",
		aot:    aot,
		caller: caller,
		bin:    erc20bin,
		abi:    erc20abi,
		txdata: txdata,
		calldatas: []CallData{
			mintCallData,
		},
		repeat: 1,
	}

	Ca(t, ca)
}

func Test_e2e_fib_non_aot(t *testing.T) {
	aot := false
	Fib(t, aot)
}

func Test_e2e_fib_aot(t *testing.T) {
	aot := true
	Fib(t, aot)
}

func Fib(t *testing.T, aot bool) {
	caller := common.HexToAddress("0xe100713fc15400d1e94096a545879e7c647001e0")

	fibbin, err := hexutil.Decode(fibca.FibonacciBin)
	require.NoError(t, err)
	fibabi, err := fibca.FibonacciMetaData.GetAbi()
	require.NoError(t, err)
	txdata, err := hexutil.Decode(fibca.FibonacciBin)
	require.NoError(t, err)

	fibdata, err := fibabi.Pack("fibonacci", big.NewInt(25))
	require.NoError(t, err)

	fibCallData := CallData{
		name:     "fibonacci()",
		calldata: fibdata,
		expected: types.Success{
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
		},
	}

	ca := TestContract{
		name:   "Fibonacci",
		aot:    aot,
		caller: caller,
		bin:    fibbin,
		abi:    fibabi,
		txdata: txdata,
		calldatas: []CallData{
			fibCallData,
		},
		repeat: 5,
	}

	Ca(t, ca)
}

func Ca(t *testing.T, ca TestContract) {
	fmt.Printf("Testing contract %s...\n", ca.name)

	vm, kvStore := setupTest(t, ca.aot, ca.caller)

	createTx := testutils.MockTx(ca.caller, common.Address{}, ca.txdata, 0)
	block := testutils.MockBlock(1)
	res, err := vm.ExecuteTx(kvStore, block.ToSerialized(), createTx.ToSerialized())
	require.NoError(t, err)
	result, err := res.ProcessExecutionResult()
	require.NoError(t, err)
	createRes, ok := result.(types.Success)
	require.True(t, ok)
	deployedAddr := createRes.Output.DeployedAddress

	nonce := uint64(1)
	for repeat := 0; repeat < ca.repeat; repeat++ {
		for i := 0; i < len(ca.calldatas); i++ {
			calldata := ca.calldatas[i]
			testTx := testutils.MockTx(ca.caller, deployedAddr, calldata.calldata, nonce)

			start := time.Now()

			res, err = vm.ExecuteTx(kvStore, block.ToSerialized(), testTx.ToSerialized())
			require.NoError(t, err)

			result, err = res.ProcessExecutionResult()
			require.NoError(t, err)

			callRes, ok := result.(types.Success)
			require.True(t, ok)

			require.Equal(t, calldata.expected, callRes)

			elapsed := time.Since(start)
			t.Logf("%s: Test %s: Call %d execution time: %v", ca.name, calldata.name, i+1, elapsed)

			time.Sleep(1 * time.Second)
			nonce++
		}
	}
}
