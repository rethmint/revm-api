package api_test

import (
	"github.com/rethmint/revm-api/api"
	"testing"
)

func TestJsonFFI(t *testing.T) {
	tx := api.MockTx{
		From:  "0xabc",
		To:    "0xdef",
		Value: "1000",
	}

	api.Json_ffi(tx)
}
