package types

import (
	"fmt"

	flatbuffers "github.com/google/flatbuffers/go"
	resulttype "github.com/rethmint/revm-api/types/go/result"
)

type Log struct {
	Address AccountAddress
	Data    LogData
}
type LogData struct {
	Topics []U256
	Data   []byte
}

type OutputType uint8

type Output struct {
	DeployedAddress [20]byte
	Output          []byte
}

type Result interface{}

// Success variant of ExecutionResult
type Success struct {
	Reason      string
	GasUsed     uint64
	GasRefunded uint64
	Logs        []Log
	Output      Output
}

// Revert variant of ExecutionResult
type Revert struct {
	GasUsed uint64
	Output  []byte
}

// Halt variant of ExecutionResult
type Halt struct {
	Reason  string
	GasUsed uint64
}

type ExecutionResult []byte

func (res ExecutionResult) ProcessExecutionResult() (Result, error) {
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
		logs := make([]Log, logLen)
		var log resulttype.Log
		for i := 0; i < logLen; i++ {
			if !unionSuccess.Logs(&log, i) {
				return nil, fmt.Errorf("failed to get log at index %d", i)
			}
			var logData resulttype.LogData
			if log.Data(&logData) == nil {
				return nil, fmt.Errorf("failed to get log data at index %d", i)
			}
			topicsLen := logData.TopicsLength()
			var topic resulttype.Topic
			topics := make([]U256, topicsLen)
			for j := 0; j < topicsLen; j++ {
				if !logData.Topics(&topic, j) {
					return nil, fmt.Errorf("failed to get log data topic at index %d", j)
				}
				var topic32 [32]byte
				copy(topic32[:], topic.ValueBytes())
				topics[j] = topic32
			}
			logs[i] = Log{
				Address: AccountAddress(log.AddressBytes()),
				Data: LogData{
					Topics: topics,
					Data:   logData.DataBytes(),
				},
			}
		}

		deployedAddr := make([]byte, 20)
		copy(deployedAddr, unionSuccess.DeployedAddressBytes())
		return Success{
			Reason:      unionSuccess.Reason().String(),
			GasUsed:     unionSuccess.GasUsed(),
			GasRefunded: unionSuccess.GasRefunded(),
			Logs:        logs,
			Output: Output{
				DeployedAddress: [20]byte(deployedAddr),
				Output:          unionSuccess.OutputBytes(),
			},
		}, nil

	case resulttype.ExResultRevert:
		unionRevert := new(resulttype.Revert)
		unionRevert.Init(resultTable.Bytes, resultTable.Pos)

		return Revert{
			GasUsed: unionRevert.GasUsed(),
			Output:  unionRevert.OutputBytes(),
		}, nil
	case resulttype.ExResultHalt:
		unionHalt := new(resulttype.Halt)
		unionHalt.Init(resultTable.Bytes, resultTable.Pos)

		return Halt{
			Reason:  unionHalt.Reason().String(),
			GasUsed: unionHalt.GasUsed(),
		}, nil
	default:
		return nil, fmt.Errorf("unknown result type: %d", evmResult.ResultType())
	}
}
