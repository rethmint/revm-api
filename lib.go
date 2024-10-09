package revm_api

import (
	flatbuffers "github.com/google/flatbuffers/go"
	"github.com/rethmint/revm-api/api"
	"github.com/rethmint/revm-api/types/go"
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
	block blockbuffer.Block,
	tx txbuffer.Transaction,
) (types.Result, error) {

	res, err := api.ExecuteTx(
		vm.Inner,
		kvStore,
		block.Table().Bytes,
		tx.Table().Bytes,
	)
	if err != nil {
		return nil, err
	}

	return processExecutionResult(res)
}

func (vm *VM) Query(
	kvStore api.KVStore,
	block blockbuffer.Block,
	tx txbuffer.Transaction,
) (types.Result, error) {
	res, err := api.Query(
		vm.Inner,
		kvStore,
		block.Table().Bytes,
		tx.Table().Bytes,
	)
	if err != nil {
		return nil, err
	}
	return processExecutionResult(res)
}

func processExecutionResult(res types.ExecutionResult) (types.Result, error) {
	evmResult := resulttype.GetRootAsEvmResult(res, 0)
	switch evmResult.ResultType() {
	case resulttype.ExResultSuccess:
		successResult := resulttype.GetRootAsSuccess(evmResult.Table().Bytes, 0)
		logLen := successResult.LogsLength()
		logs := make([]types.Log, logLen)
		var log resulttype.Log
		for i := 0; i < logLen; i++ {
			successResult.Logs(&log, i)
			var logData resulttype.LogData
			topicsLen := logData.TopicsLength()
			var topic resulttype.Topic
			topics := make([]types.U256, topicsLen)
			for j := 0; j < topicsLen; j++ {
				logData.Topics(&topic, j)
				topics[j] = types.BytesToU256(topic.ValueBytes())
			}
			logs[i] = types.Log{
				Address: types.AccountAddress(log.AddressBytes()),
				Data: types.LogData{
					Topics: topics,
					Data:   logData.DataBytes(),
				},
			}
		}
		var output types.Output
		outputTable := new(flatbuffers.Table)
		successResult.Output(outputTable)
		switch successResult.OutputType() {
		case resulttype.OutputCall:
			outputCall := new(resulttype.Call)
			outputCall.Init(outputTable.Bytes, outputTable.Pos)
			output = types.Output{
				DeployedAddress: [20]byte{0},
				Output:          outputCall.OutputBytes(),
			}
		case resulttype.OutputCreate:
			outputCall := new(resulttype.Create)
			outputCall.Init(outputTable.Bytes, outputTable.Pos)
			output = types.Output{
				DeployedAddress: [20]byte(outputCall.AddressBytes()),
				Output:          outputCall.OutputBytes(),
			}
		default:
			return nil, nil
		}

		return types.Success{
			Reason:      successResult.Reason().String(),
			GasUsed:     successResult.GasUsed(),
			GasRefunded: successResult.GasRefunded(),
			Logs:        logs,
			Output:      output,
		}, nil
	case resulttype.ExResultRevert:
		revertResult := resulttype.GetRootAsRevert(evmResult.Table().Bytes, 0)
		return types.Revert{
			GasUsed: revertResult.GasUsed(),
			Output:  revertResult.OutputBytes(),
		}, nil
	case resulttype.ExResultHalt:
		haltResult := resulttype.GetRootAsHalt(evmResult.Table().Bytes, 0)
		return types.Halt{
			Reason:  haltResult.Reason().String(),
			GasUsed: haltResult.GasUsed(),
		}, nil
	default:
		return nil, nil
	}
}
