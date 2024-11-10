package testutils

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	types "github.com/rethmint/revm-api/types/go"
)

func MockTx(caller common.Address, transactTo [20]byte, txData []byte, nonce uint64) types.TransactionEnv {
	return types.TransactionEnv{
		Caller:         caller,
		GasLimit:       1000000,
		GasPrice:       common.BigToHash(big.NewInt(10000)),
		TransactTo:     transactTo,
		Value:          common.BigToHash(big.NewInt(0)),
		Data:           txData,
		Nonce:          nonce,
		ChainId:        1,
		GasPriorityFee: common.BigToHash(big.NewInt(0)),
		AccessList: types.AccessList{
			common.Address{}: []common.Hash{common.BigToHash(big.NewInt(0))},
		},
	}
}

func MockBlock(num int64) types.BlockEnv {
	return types.BlockEnv{
		Number:    common.BigToHash(big.NewInt(num)),
		Coinbase:  common.Address{},
		Timestamp: common.BigToHash(big.NewInt(1000000)),
		GasLimit:  common.BigToHash(big.NewInt(10000000)),
		Basefee:   common.BigToHash(big.NewInt(0)),
	}

}
