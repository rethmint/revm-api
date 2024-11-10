package revmffi_test

import (
	"encoding/hex"
	"fmt"

	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	revm "github.com/rethmint/revm-api"
	"github.com/rethmint/revm-api/testutils"
	revmapitypes "github.com/rethmint/revm-api/types/go"
)


func extractTxData(txStr string) []byte {
	if txStr[:2] == "0x" {
		txStr = txStr[2:]
	}

	txData, err := hex.DecodeString(txStr)
	if err != nil {
		panic(err)
	}

	return txData
}

func call(vm *revm.VM, contractAddress common.Address, abi *abi.ABI, method string, kvStore *testutils.MockKVStore, caller common.Address, args ...interface{}) ([]byte, error) {
	callData, err := abi.Pack(method, args...)
	if err != nil {
		return nil, fmt.Errorf("failed to pack method call: %v", err)
	}

	accountInfo := testutils.ExtractAccountInfo(kvStore, caller)

	tx := testutils.MockTx(caller, contractAddress, callData, accountInfo.Nonce)
	block := testutils.MockBlock(1000)
	serializedBlock := block.ToSerialized()
	serializedTx := tx.ToSerialized()

	res, err := vm.ExecuteTx(kvStore, serializedBlock, serializedTx)
	if err != nil {
		return nil, fmt.Errorf("transaction execution failed: %v", err)
	}

	processedRes, _ := res.ProcessExecutionResult()

	if successRes, ok := processedRes.(revmapitypes.Success); ok {
		return successRes.Output.Output, nil
	}
	return nil, fmt.Errorf("contract call failed")
}

func deploy(vm *revm.VM, bin string, abi *abi.ABI, kvStore *testutils.MockKVStore, caller common.Address, args ...interface{}) common.Address {
	accountInfo := testutils.ExtractAccountInfo(kvStore, caller)
	bytecode := extractTxData(bin)

	argsBytes, err := abi.Constructor.Inputs.Pack(args)
	if err != nil {
		panic(err)
	}
	txData := append(bytecode, argsBytes...)
	tx := testutils.MockTx(caller, common.Address{}, txData, accountInfo.Nonce)
	block := testutils.MockBlock(1000)
	serializedBlock := block.ToSerialized()
	serializedTx := tx.ToSerialized()

	res, err := vm.ExecuteTx(kvStore, serializedBlock, serializedTx)
	if err != nil {
		panic("Execute failed\n")
	}

	processedRes, _ := res.ProcessExecutionResult()
	successRes, ok := processedRes.(revmapitypes.Success)
	if !ok {
		panic("Deploy failed\n")
	}
	return successRes.Output.DeployedAddress
}
