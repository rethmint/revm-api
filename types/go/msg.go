package types

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

type SuccessReason uint

const (
	Stop SuccessReason = iota
	Return
	SelfDestruct
	EofReturnContract
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

type ResultId uint8

const (
	SuccessId ResultId = iota
	RevertId
	HaltId
	ErrorId
)

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

func (result ExecutionResult) ResultId() ResultId {
	return ResultId(result[0])
}
