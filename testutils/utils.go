package testutils

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
)

var AccountPrefix = []byte{0x01}

func Faucet(kvStore *MockKVStore, address common.Address, amount *big.Int) {
	accountKey := append(AccountPrefix, address.Bytes()...)
	var accountInfoBytes []byte
	if accountInfoBytes = kvStore.Get(accountKey); accountInfoBytes == nil {
		accountInfoBytes, _ = kvStore.CreateEOA(accountKey)
	}

	accountInfo, _ := AccountInfoFromBytes(accountInfoBytes)
	accountInfo.Balance = accountInfo.Balance.Add(accountInfo.Balance, amount)

	kvStore.Set(accountKey, accountInfo.ToBytes())
}
