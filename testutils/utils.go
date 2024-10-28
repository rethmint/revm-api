package testutils

import (
	"math/big"

	types "github.com/rethmint/revm-api/types/go"
)

var AccountPrefix = [1]byte{0x01}
var CreateTransactTo = [20]uint8{0}

type AccountAddressKey []byte

func AddressToAccountAddressKey(address types.AccountAddress) AccountAddressKey {
	result := make([]byte, 1)
	copy(result[:], AccountPrefix[:])

	result = append(result, address[:]...)

	return result
}

func Faucet(kvStore *MockKVStore, address types.AccountAddress, amount *big.Int) {
	accountKey := AddressToAccountAddressKey(address)

	var accountInfoBytes []byte
	if accountInfoBytes = kvStore.Get(accountKey); accountInfoBytes == nil {
		accountInfoBytes, _ = kvStore.CreateEOA(accountKey)
	}

	accountInfo, _ := AccountInfoFromBytes(accountInfoBytes)
	accountInfo.Balance = accountInfo.Balance.Add(accountInfo.Balance, amount)

	kvStore.Set(accountKey, accountInfo.ToBytes())
}


func DefaultTx(caller types.AccountAddress, transactTo [20]byte, txData []byte, nonce uint64) types.TransactionEnv {
	return types.TransactionEnv{
		Caller:         caller,
		GasLimit:       1000000,
		GasPrice:       types.NewU256(big.NewInt(10000)),
		TransactTo:     transactTo,
		Value:          types.NewU256(big.NewInt(0)),
		Data:           txData,
		Nonce:          nonce,
		ChainId:        1,
		GasPriorityFee: types.NewU256(big.NewInt(0)),
		AccessList: types.AccessList{
			types.ZeroAddress(): []types.U256{types.NewU256(big.NewInt(0))},
		},
	}
}

func DefaultBlock(num int64) types.BlockEnv {
	return types.BlockEnv{
		Number:    types.NewU256(big.NewInt(num)),
		Coinbase:  types.ZeroAddress(),
		Timestamp: types.NewU256(big.NewInt(1000000)),
		GasLimit:  types.NewU256(big.NewInt(10000000)),
		Basefee:   types.NewU256(big.NewInt(0)),
	}

}