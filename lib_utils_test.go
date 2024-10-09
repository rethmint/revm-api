package revm_api_test

import (
	"encoding/binary"
	"encoding/hex"
	"math/big"
	"math/rand"
	"strings"
	"testing"

	"github.com/ethereum/go-ethereum/accounts/abi"
	revm "github.com/rethmint/revm-api"
	api "github.com/rethmint/revm-api/api"
	types "github.com/rethmint/revm-api/types"
)

func generateRandomHash() [32]byte {
	bz := make([]byte, 0, 32)
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())

	var resBz [32]byte
	copy(resBz[:], bz)

	return resBz
}

func startVM(t *testing.T) (revm.VM, *api.MockKVStore) {
	kvStore := api.NewMockKVStore()
	vm := revm.NewVM()
	t.Cleanup(func() {
		vm.Destroy()
	})
	return vm, kvStore
}

var AccountPrefix = [1]byte{0x01}
var CreateTransactTo = [20]uint8{0}

type AccountAddressKey []byte

func AddressToAccountAddressKey(address types.AccountAddress) AccountAddressKey {
	result := make([]byte, 1)
	copy(result[:], AccountPrefix[:])

	result = append(result, address[:]...)

	return result
}

func faucet(kvStore *api.MockKVStore, address types.AccountAddress, amount *big.Int) {
	accountKey := AddressToAccountAddressKey(address)

	var accountInfoBytes []byte
	if accountInfoBytes = kvStore.Get(accountKey); accountInfoBytes == nil {
		accountInfoBytes, _ = kvStore.CreateEOA(accountKey)
	}

	accountInfo, _ := types.AccountInfoFromBytes(accountInfoBytes)
	accountInfo.Balance = accountInfo.Balance.Add(accountInfo.Balance, amount)

	kvStore.Set(accountKey, accountInfo.ToBytes())
}

func extractTxData(t *testing.T, txStr string) []byte {
	if txStr[:2] == "0x" {
		txStr = txStr[2:]
	}

	txData, err := hex.DecodeString(txStr)
	if err != nil {
		t.Fatal(err)
	}

	return txData
}

func extractCallData(t *testing.T, abi abi.ABI, method string) []byte {
	callData, err := abi.Pack(method)
	if err != nil {
		t.Fatalf("Failed to pack method call: %v", err)
	}
	return callData
}

func defaultTx(caller types.AccountAddress, transactTo [20]byte, nonce uint64) types.Transaction {
	return types.Transaction{
		Caller:         caller,
		GasLimit:       0x100000000,
		GasPrice:       big.NewInt(10),
		TransactTo:     transactTo,
		Value:          big.NewInt(0),
		Nonce:          nonce,
		ChainId:        1,
		GasPriorityFee: big.NewInt(10),
	}
}

func defaultBlock() types.Block {
	return types.Block{
		Number:    big.NewInt(1),
		Coinbase:  types.ZeroAddress(),
		Timestamp: big.NewInt(0),
		GasLimit:  big.NewInt(10000000000),
		Basefee:   big.NewInt(0),
	}

}

func parseABI(t *testing.T, dest_abi string) abi.ABI {
	parsedABI, err := abi.JSON(strings.NewReader(dest_abi))
	if err != nil {
		t.Fatalf("Failed to parse ABI: %v", err)
	}

	return parsedABI
}
