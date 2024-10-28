package testutils

import (
	"bytes"
	"encoding/binary"
	"math/big"
)

// 72 Byte
type AccountInfo struct {
	Balance  *big.Int
	Nonce    uint64
	CodeHash [32]byte
}

func (accountInfo AccountInfo) ToBytes() []byte {
	var result bytes.Buffer

	balanceBytes := accountInfo.Balance.Bytes()
	paddedBalance := make([]byte, 32)
	copy(paddedBalance[:], balanceBytes)
	result.Write(paddedBalance)

	nonceBytes := make([]byte, 8)
	binary.BigEndian.PutUint64(nonceBytes, accountInfo.Nonce)
	result.Write(nonceBytes)

	result.Write(accountInfo.CodeHash[:])

	return result.Bytes()
}

func AccountInfoFromBytes(data []byte) (AccountInfo, error) {
	var account AccountInfo
	balance := new(big.Int)
	balance.SetBytes(data[:32])
	account.Balance = balance
	account.Nonce = binary.BigEndian.Uint64(data[32:40])
	copy(account.CodeHash[:], data[40:])

	return account, nil
}

type Bytecode []byte
