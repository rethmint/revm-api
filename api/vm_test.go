package api_test

import (
	"encoding/hex"
	"fmt"
	"math/big"
	"testing"

	"github.com/rethmint/revm-api/api"
	types "github.com/rethmint/revm-api/types"
)

func TestJsonFFI(t *testing.T) {
	tx := api.MockTx{
		From:  "0xabc",
		To:    "0xdef",
		Value: "1000",
	}

	api.Json_ffi(tx)
}

func TestJsonFFIBlockEnv(t *testing.T) {
	block := types.Block{
		Number:    big.NewInt(1),
		Coinbase:  types.AccountAddress{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
		Timestamp: big.NewInt(0),
		GasLimit:  big.NewInt(1),
		Basefee:   big.NewInt(0),
	}

	api.Json_ffi_block(block)
}

// evm  suite test case for contract deployment
func TestExecuteTx(t *testing.T) {
	store := api.NewRevmKVStore()
	vm := api.InitVM()

	hexStr := "60fe60005360016000f3"

	byteSlice, err := hex.DecodeString(hexStr)
	if err != nil {
		fmt.Println("Error decoding hex string:", err)
		return
	}
	var byteArray [32]byte
	copy(byteArray[:], byteSlice)

	tx := types.Transaction{
		Caller:         types.AccountAddress{0xe1, 0x00, 0x71, 0x3f, 0xc1, 0x54, 0x00, 0xd1, 0xe9, 0x40, 0x96, 0xa5, 0x45, 0x87, 0x9e, 0x7c, 0x64, 0x70, 0x01, 0xe0},
		GasLimit:       0xf4240,
		GasPrice:       big.NewInt(1000),
		TransactTo:     [20]uint8{0},
		Value:          big.NewInt(0),
		Data:           byteArray,
		Nonce:          1,
		ChainId:        1,
		GasPriorityFee: big.NewInt(1000),
	}

	block := types.Block{
		Number:    big.NewInt(1),
		Coinbase:  types.AccountAddress{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
		Timestamp: big.NewInt(0),
		GasLimit:  big.NewInt(1),
		Basefee:   big.NewInt(0),
	}

	res, _ := api.ExecuteTx(vm, store, tx, block)
	fmt.Println("Res: \n", res)
}
