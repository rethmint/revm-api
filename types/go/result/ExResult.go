// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package result

import "strconv"

type ExResult byte

const (
	ExResultNONE    ExResult = 0
	ExResultSuccess ExResult = 1
	ExResultRevert  ExResult = 2
	ExResultHalt    ExResult = 3
	ExResultError   ExResult = 4
)

var EnumNamesExResult = map[ExResult]string{
	ExResultNONE:    "NONE",
	ExResultSuccess: "Success",
	ExResultRevert:  "Revert",
	ExResultHalt:    "Halt",
	ExResultError:   "Error",
}

var EnumValuesExResult = map[string]ExResult{
	"NONE":    ExResultNONE,
	"Success": ExResultSuccess,
	"Revert":  ExResultRevert,
	"Halt":    ExResultHalt,
	"Error":   ExResultError,
}

func (v ExResult) String() string {
	if s, ok := EnumNamesExResult[v]; ok {
		return s
	}
	return "ExResult(" + strconv.FormatInt(int64(v), 10) + ")"
}
