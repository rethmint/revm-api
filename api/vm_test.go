package api_test

import (
	"encoding/hex"
	"fmt"
	"math/big"
	"testing"

	"github.com/rethmint/revm-api/api"
	types "github.com/rethmint/revm-api/types"
)

// evm  suite test case for contract deployment
func TestExecuteTx(t *testing.T) {
	store := api.NewRevmKVStore()
	vm := api.InitVM()

	txStr := "60fe60005360016000f3"

	txData, err := hex.DecodeString(txStr)
	if err != nil {
		fmt.Println("Error decoding hex string:", err)
		return
	}
	caller, _ := types.NewAccountAddress("0xe100713fc15400d1e94096a545879e7c647001e0")
	tx := types.Transaction{
		Caller:         caller,
		GasLimit:       0xf4240,
		GasPrice:       big.NewInt(1000),
		TransactTo:     [20]uint8{0},
		Value:          big.NewInt(0),
		Nonce:          1,
		ChainId:        1,
		GasPriorityFee: big.NewInt(1000),
	}

	block := types.Block{
		Number:    big.NewInt(1),
		Coinbase:  types.ZeroAddress(),
		Timestamp: big.NewInt(0),
		GasLimit:  big.NewInt(1),
		Basefee:   big.NewInt(0),
	}

	res, _ := api.ExecuteTx(vm, store, block, tx, txData)
	fmt.Println("Res: \n", res)
}
