package gevm_test

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/state"
	evmtypes "github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/params"
	"github.com/ethereum/go-ethereum/triedb"
	"github.com/holiman/uint256"
	"github.com/rethmint/revm-api/contracts/erc20"
	"github.com/stretchr/testify/require"
)

const caller = "0x10"
const gaslimit = 3000000

func Test_ERC20_Benchmark(t *testing.T) {
	erc20abi, _ := erc20.Erc20MetaData.GetAbi()
	erc20bin, _ := hexutil.Decode(erc20.Erc20Bin)
	callerAddr := common.HexToAddress(caller)
	// Create VM
	config := params.MainnetChainConfig
	genesis := &core.Genesis{
		Config:     config,
		Coinbase:   common.Address{},
		Difficulty: big.NewInt(0),
		GasLimit:   gaslimit,
		Number:     config.LondonBlock.Uint64(),
		Timestamp:  *config.CancunTime,
		Alloc:      evmtypes.GenesisAlloc{},
	}
	blockContext := core.NewEVMBlockContext(genesis.ToBlock().Header(), nil, &common.Address{})
	txContext := core.NewEVMTxContext(&core.Message{
		From:       common.HexToAddress(caller),
		To:         &common.Address{},
		Nonce:      0,
		GasLimit:   gaslimit,
		GasPrice:   big.NewInt(10000),
		GasFeeCap:  big.NewInt(0),
		GasTipCap:  big.NewInt(0),
		Data:       erc20bin,
		AccessList: evmtypes.AccessList{},
	})

	memDb := rawdb.NewMemoryDatabase()
	trieDb := triedb.NewDatabase(memDb, nil)
	statedb, _ := state.New(common.Hash{}, state.NewDatabase(trieDb, nil))
	evm := vm.NewEVM(blockContext, txContext, statedb, config, vm.Config{})

	// ERC20 create
	packedData, err := erc20abi.Constructor.Inputs.Pack("Mock Token", "Mock")
	require.NoError(t, err)
	calldata := append(erc20bin, packedData...)
	_, contractAddress, _, err := evm.Create(vm.AccountRef(callerAddr), calldata, gaslimit, new(uint256.Int))
	require.NoError(t, err)

	// ERC20 Mint
	mintData, _ := erc20abi.Pack("mint", callerAddr, big.NewInt(1000))
	_, _, err = evm.Call(vm.AccountRef(callerAddr), contractAddress, mintData, gaslimit, new(uint256.Int))
	require.NoError(t, err)

	// ERC20 Transfer
	recipientAddr := common.HexToAddress("0x20")
	transferData, _ := erc20abi.Pack("transfer", recipientAddr, big.NewInt(100))
	_, _, err = evm.Call(vm.AccountRef(callerAddr), contractAddress, transferData, gaslimit, new(uint256.Int))
	require.NoError(t, err)

	// ERC20 BalanceOf
	balanceOfData, _ := erc20abi.Pack("balanceOf", recipientAddr)
	_, _, err = evm.StaticCall(vm.AccountRef(callerAddr), contractAddress, balanceOfData, gaslimit)
	require.NoError(t, err)

}
