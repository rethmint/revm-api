package revm_api

import (
	"fmt"

	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/rethmint/revm-api/api"
	types "github.com/rethmint/revm-api/types/go"
	blockbuffer "github.com/rethmint/revm-api/types/go/block"
	resulttype "github.com/rethmint/revm-api/types/go/result"
	txbuffer "github.com/rethmint/revm-api/types/go/transaction"
)

// VM struct is the core of initiavm.
type VM struct {
	Inner api.VM
}

// NewVm return VM instance
// handler
func NewVM() VM {
	inner := api.InitVM()
	return VM{inner}
}

func (vm *VM) Destroy() {
	api.ReleaseVM(vm.Inner)
}

// ExecuteTx execute transaction based on revm
// for bootstrapping genesis
func (vm *VM) ExecuteTx(
	kvStore api.KVStore,
	block types.Block,
	tx types.Transaction,
) (types.Result, error) {

	res, err := api.ExecuteTx(
		vm.Inner,
		kvStore,
		serializeBlock(block),
		serializeTransaction(tx),
	)
	if err != nil {
		return nil, err
	}

	return processExecutionResult(res)
}

func (vm *VM) Query(
	kvStore api.KVStore,
	block types.Block,
	tx types.Transaction,
) (types.Result, error) {
	res, err := api.Query(
		vm.Inner,
		kvStore,
		serializeBlock(block),
		serializeTransaction(tx),
	)
	if err != nil {
		return nil, err
	}

	return processExecutionResult(res)
}

func serializeBlock(block types.Block) []byte {
	builder := flatbuffers.NewBuilder(164)
	number := builder.CreateByteVector(block.Number[:])
	coinbase := builder.CreateByteVector(block.Coinbase[:])
	timeStamp := builder.CreateByteVector(block.Timestamp[:])
	gasLimit := builder.CreateByteVector(block.GasLimit[:])
	baseFee := builder.CreateByteVector(block.Basefee[:])
	blockbuffer.BlockStart(builder)
	blockbuffer.BlockAddNumber(builder, number)       // 32
	blockbuffer.BlockAddCoinbase(builder, coinbase)   // 20
	blockbuffer.BlockAddTimestamp(builder, timeStamp) // 32
	blockbuffer.BlockAddGasLimit(builder, gasLimit)   // 32
	blockbuffer.BlockAddBasefee(builder, baseFee)     //32
	offset := blockbuffer.BlockEnd(builder)
	builder.Finish(offset)
	return builder.FinishedBytes()
}

func serializeTransaction(transaction types.Transaction) []byte {
	builder := flatbuffers.NewBuilder(244)
	caller := builder.CreateByteVector(transaction.Caller[:])
	gasPrice := builder.CreateByteVector(transaction.GasPrice[:])
	transactTo := builder.CreateByteVector(transaction.TransactTo[:])
	value := builder.CreateByteVector(transaction.Value[:])
	txData := builder.CreateByteVector(transaction.Data[:])
	gasPriorityFee := builder.CreateByteVector(transaction.GasPriorityFee[:])

	txbuffer.TransactionStart(builder)
	txbuffer.TransactionAddCaller(builder, caller)                 // 20
	txbuffer.TransactionAddGasLimit(builder, transaction.GasLimit) // 32
	txbuffer.TransactionAddGasPrice(builder, gasPrice)             // 8
	txbuffer.TransactionAddTransactTo(builder, transactTo)         // 20
	txbuffer.TransactionAddValue(builder, value)                   // 32
	txbuffer.TransactionAddData(builder, txData)                   // estimate: 100 byte -> variable
	txbuffer.TransactionAddGasPriorityFee(builder, gasPriorityFee) // 32
	offset := txbuffer.TransactionEnd(builder)
	builder.Finish(offset)
	return builder.FinishedBytes()
}

func processExecutionResult(res types.ExecutionResult) (types.Result, error) {
	evmResult := resulttype.GetRootAsEvmResult(res, 0)
	resultTable := new(flatbuffers.Table)
	if !evmResult.Result(resultTable) {
		return nil, fmt.Errorf("failed to get result from evmResult")
	}
	switch evmResult.ResultType() {
	case resulttype.ExResultSuccess:
		unionSuccess := new(resulttype.Success)
		unionSuccess.Init(resultTable.Bytes, resultTable.Pos)

		logLen := unionSuccess.LogsLength()
		logs := make([]types.Log, logLen)
		var log resulttype.Log
		for i := 0; i < logLen; i++ {
			if !unionSuccess.Logs(&log, i) {
				return nil, fmt.Errorf("failed to get log at index %d", i)
			}
			var logData resulttype.LogData
			topicsLen := logData.TopicsLength()
			var topic resulttype.Topic
			topics := make([]types.U256, topicsLen)
			for j := 0; j < topicsLen; j++ {
				if !logData.Topics(&topic, j) {
					return nil, fmt.Errorf("failed to get log data topic at index %d", j)
				}
				var topic32 [32]byte
				copy(topic32[:], topic.ValueBytes())
				topics[j] = topic32
			}
			logs[i] = types.Log{
				Address: types.AccountAddress(log.AddressBytes()),
				Data: types.LogData{
					Topics: topics,
					Data:   logData.DataBytes(),
				},
			}
		}

		deployedAddr := make([]byte, 20)
		copy(deployedAddr, unionSuccess.DeployedAddressBytes())
		return types.Success{
			Reason:      unionSuccess.Reason().String(),
			GasUsed:     unionSuccess.GasUsed(),
			GasRefunded: unionSuccess.GasRefunded(),
			Logs:        logs,
			Output: types.Output{
				DeployedAddress: [20]byte(deployedAddr),
				Output:          unionSuccess.OutputBytes(),
			},
		}, nil

	case resulttype.ExResultRevert:
		unionRevert := new(resulttype.Revert)
		unionRevert.Init(resultTable.Bytes, resultTable.Pos)

		return types.Revert{
			GasUsed: unionRevert.GasUsed(),
			Output:  unionRevert.OutputBytes(),
		}, nil
	case resulttype.ExResultHalt:
		unionHalt := new(resulttype.Halt)
		unionHalt.Init(resultTable.Bytes, resultTable.Pos)

		return types.Halt{
			Reason:  unionHalt.Reason().String(),
			GasUsed: unionHalt.GasUsed(),
		}, nil
	default:
		return nil, fmt.Errorf("unknown result type: %d", evmResult.ResultType())
	}
}
