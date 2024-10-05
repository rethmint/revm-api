package types

import (
	"fmt"
	"math/big"
)

type AccountInfo struct {
	Balance  *big.Int
	Nonce    uint64
	CodeHash [32]byte
}

type Bytecode []byte

func main() {
	account := AccountInfo{
		Balance:  big.NewInt(1000),
		Nonce:    1,
		CodeHash: [32]byte{},
		Code:     nil,
	}

	fmt.Printf("%+v\n", account)
}
