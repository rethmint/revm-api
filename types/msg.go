package types

import (
	"encoding/json"
	"fmt"
)

type HaltReason uint

const (
	OutOfGas HaltReason = iota
	OpcodeNotFound
	InvalidFEOpcode
	InvalidJump
	NotActivated
	StackUnderflow
	StackOverflow
	OutOfOffset
	CreateCollision
	PrecompileError
	NonceOverflow
	CreateContractSizeLimit
	CreateContractStartingWithEF
	CreateInitCodeSizeLimit

	// Internal Halts
	OverflowPayment
	StateChangeDuringStaticCall
	CallNotAllowedInsideStatic
	OutOfFunds
	CallTooDeep

	EOFSubroutineStackOverflow
	InvalidEXTCALLTarget
	EofAuxDataOverflow
	EofAuxDataTooSmall
)

// HaltReasonTrait will be represented as an interface in Go
type HaltReasonTrait interface {
	Reason() string
}

type SuccessReason uint

const (
	Stop SuccessReason = iota
	Return
	SelfDestruct
	EofReturnContract
)

// Log is a placeholder. Define this as per your actual requirements.
type Log struct {
	Address AccountAddress
	Data    LogData
}
type LogData struct {
	Topics []U256
	Data   []byte
}

// Output is a placeholder. Define this as per your actual requirements.
type Output []byte
type ResultId uint8

const (
	SuccessId ResultId = iota
	RevertId
	HaltId
	ErrorId
)

// Success variant of ExecutionResult
type Success struct {
	Reason      SuccessReason
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
	Reason  HaltReasonTrait
	GasUsed uint64
}

type ExecutionResult []byte

func (result ExecutionResult) ResultId() ResultId {
	return ResultId(result[0])
}

type ResultData interface{}

func (result ExecutionResult) ResultData() (ResultData, error) {
	data := result[1:]
	switch result.ResultId() {
	case SuccessId:
		var success Success
		if err := json.Unmarshal(data, &success); err != nil {
			return nil, err
		}
		return success, nil
	case RevertId:
		var revert Revert
		if err := json.Unmarshal(data, &revert); err != nil {
			return nil, err
		}
		return revert, nil
	case HaltId:
		var halt Halt
		if err := json.Unmarshal(data, &halt); err != nil {
			return nil, err
		}
		return halt, nil
	case ErrorId:
		return map[string]string{"error": "execution resulted in an error"}, nil
	default:
		return nil, fmt.Errorf("unknown result id")
	}
}
