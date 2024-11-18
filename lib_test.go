package revm_api_test

import (
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	revm "github.com/rethmint/revm-api"
	fibca "github.com/rethmint/revm-api/contracts/Fibonacci"
	// testca "github.com/rethmint/revm-api/contracts/Test"
	testutils "github.com/rethmint/revm-api/testutils"
	types "github.com/rethmint/revm-api/types/go"
	"github.com/stretchr/testify/require"
)

const CANCUN uint8 = 17

func setupTest(t *testing.T, aot bool) (revm.VM, *testutils.MockKVStore, common.Address) {
	kvStore := testutils.NewMockKVStore()
	vm := revm.NewVM(CANCUN)

	var compiler revm.Compiler
	if aot {
		compiler = revm.NewCompiler()
		go func() {
			compiler.Start(kvStore)
		}()
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

// func Test_e2e(t *testing.T) {
// 	vm, kvStore, caller := setupTest(t, false)
//
// 	// Deploy Test Contract
// 	txData, err := hexutil.Decode(testca.TestBin)
// 	require.NoError(t, err)
// 	createTx := testutils.MockTx(caller, common.Address{}, txData, 0)
// 	block := testutils.MockBlock(1)
// 	res, err := vm.ExecuteTx(kvStore, block.ToSerialized(), createTx.ToSerialized())
// 	require.NoError(t, err)
// 	result, err := res.ProcessExecutionResult()
// 	require.NoError(t, err)
// 	createRes, ok := result.(types.Success)
// 	require.True(t, ok)
// 	deployedAddr := createRes.Output.DeployedAddress
//
// 	abi, err := testca.TestMetaData.GetAbi()
// 	require.NoError(t, err)
// 	increaseInput, err := abi.Pack("increase")
// 	require.NoError(t, err)
//
// 	increaseTx := testutils.MockTx(caller, deployedAddr, increaseInput, 1)
// 	block = testutils.MockBlock(2)
// 	res, err = vm.ExecuteTx(kvStore, block.ToSerialized(), increaseTx.ToSerialized())
// 	require.NoError(t, err)
//
// 	increaseRes, ok := result.(types.Success)
// 	require.Equal(t, types.Success{
// 		Reason:      "Stop",
// 		GasUsed:     25516,
// 		GasRefunded: 0,
// 		Logs: []types.Log{
// 			{
// 				Address: deployedAddr,
// 				Data: types.LogData{
// 					Topics: []common.Hash{ // keccack256(increased(uint256,uint256))
// 						{0x61, 0x99, 0x6f, 0xe1, 0x96, 0xf7, 0x2c, 0xb5, 0x98, 0xc4, 0x83, 0xe8, 0x96, 0xa1, 0x22, 0x12, 0x63, 0xa2, 0x8b, 0xb6, 0x30, 0x48, 0x0a, 0xa8, 0x94, 0x95, 0xf7, 0x37, 0xd4, 0xa8, 0xe3, 0xdf},
// 					},
// 					Data: []byte{
// 						0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
// 					},
// 				},
// 			},
// 		},
// 		Output: types.Output{
// 			DeployedAddress: [20]byte{},
// 			Output:          []byte{},
// 		},
// 	}, increaseRes)
//
// 	// Query
// 	countQuery, err := abi.Pack("count")
// 	require.NoError(t, err)
// 	query := testutils.MockTx(caller, deployedAddr, countQuery, 2)
// 	block = testutils.MockBlock(2)
// 	res, err = vm.QueryTx(kvStore, block.ToSerialized(), query.ToSerialized())
// 	require.NoError(t, err)
//
// 	result, err = res.ProcessExecutionResult()
// 	require.NoError(t, err)
//
// 	queryRes, ok := result.(types.Success)
// 	require.True(t, ok)
// 	require.Equal(t, types.Success{
// 		Reason:      "Return",
// 		GasUsed:     27766,
// 		GasRefunded: 0,
// 		Logs:        []types.Log{},
// 		Output: types.Output{
// 			DeployedAddress: common.Address{},
// 			Output: []byte{
// 				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
// 			},
// 		},
// 	}, queryRes)
// }

func Test_e2e_aot(t *testing.T) {
	vm, kvStore, caller := setupTest(t, true)

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
		testInput, err := abi.Pack("fibonacci", big.NewInt(30))
		require.NoError(t, err)

		start := time.Now()

		testTx := testutils.MockTx(caller, deployedAddr, testInput, uint64(i+1))
		block = testutils.MockBlock(int64(2 + i))
		res, err = vm.ExecuteTx(kvStore, block.ToSerialized(), testTx.ToSerialized())
		require.NoError(t, err)

		result, err = res.ProcessExecutionResult()
		require.NoError(t, err)

		_, ok := result.(types.Success)
		require.True(t, ok)

		elapsed := time.Since(start)
		t.Logf("Test Aot: Call %d execution time: %v", i+1, elapsed)

		time.Sleep(1 * time.Second)
	}
}
