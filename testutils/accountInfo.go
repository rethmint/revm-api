package testutils

import (
	"bytes"
	"encoding/binary"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
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

func ExtractAccountInfo(kvStore *MockKVStore, caller common.Address) AccountInfo {
	accountKey := append(AccountPrefix, caller.Bytes()...)
	accountBytes := kvStore.Get(accountKey)
	if accountBytes != nil {
		accountInfo, _ := AccountInfoFromBytes(accountBytes)
		return accountInfo
	}

	return AccountInfo{Nonce: 0}
}

type Bytecode []byte
