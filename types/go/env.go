package types

import (
	"encoding/hex"
	"encoding/json"
	"errors"
	"log"
	"math/big"
	"strings"
)

type AccountAddress [20]uint8

func NewAccountAddress(s string) (AccountAddress, error) {
	if !strings.HasPrefix(s, "0x") {
		return AccountAddress{}, errors.New("address must start with 0x")
	}
	if len(s) != 42 { // 2 characters for "0x" and 40 characters for the address
		return AccountAddress{}, errors.New("address must be 20 bytes long")
	}
	bytes, err := hex.DecodeString(s[2:])
	if err != nil {
		return AccountAddress{}, err
	}
	var address AccountAddress
	copy(address[:], bytes)
	return address, nil
}
func ZeroAddress() AccountAddress {
	var zero AccountAddress
	return zero
}

type U256 *big.Int

func BytesToU256(b []byte) U256 {
	u256 := new(big.Int)
	u256.SetBytes(b)
	return u256
}

// type TxKind interface {
// 	IsTxKind()
// }
//
// type Create struct{}
//
// type Call struct {
// 	Address AccountAddress
// }
//
// func (Create) IsTxKind() {}
// func (Call) IsTxKind()   {}

type Block struct {
	/// The number of ancestor blocks of this block (block height).
	Number U256 `json:"number"`
	/// Coinbase or miner or address that created and signed the block.
	///
	/// This is the receiver address of all the gas spent in the block.
	Coinbase AccountAddress `json:"coinbase"`

	/// The timestamp of the block in seconds since the UNIX epoch.
	Timestamp U256 `json:"timestamp"`
	/// The gas limit of the block.
	GasLimit U256 `json:"gas_limit"`
	/// The base fee per gas added in the London upgrade with [EIP-1559].
	///
	/// [EIP-1559]: https://eips.ethereum.org/EIPS/eip-1559
	Basefee U256 `json:"basefee"`
	/// The difficulty of the block.
	///
	/// Unused after the Paris (AKA the merge) upgrade and replaced by `prevrandao`.
	// difficulty: U256
	// /// The output of the randomness beacon provided by the beacon chain.
	// ///
	// /// Replaces `difficulty` after the Paris (AKA the merge) upgrade with [EIP-4399].
	// ///
	// /// NOTE: `prevrandao` can be found in a block in place of `mix_hash`.
	// ///
	// /// [EIP-4399]: https://eips.ethereum.org/EIPS/eip-4399
	// prevrandao: Option<B256>
	// /// Excess blob gas and blob gasprice.
	// /// See also [`crate::calc_excess_blob_gas`]
	// /// and [`calc_blob_gasprice`].
	// ///
	// /// Incorporated as part of the Cancun upgrade via [EIP-4844].
	// ///
	// /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
	// blob_excess_gas_and_price: Option<BlobExcessGasAndPrice>
}
type Transaction struct {
	/// Caller aka Author aka transaction signer.
	Caller AccountAddress `json:"caller"`
	/// The gas limit of the transaction.
	GasLimit uint64 `json:"gas_limit"`
	/// The gas price of the transaction.
	GasPrice U256 `json:"gas_price"`
	/// The destination of the transaction.
	TransactTo AccountAddress `json:"transact_to"`
	/// The value sent to `transact_to`.
	Value U256 `json:"value"`

	Data TransactionData `json:"data"`
	/// The nonce of the transaction.
	Nonce uint64 `json:"nonce"`

	/// The chain ID of the transaction. If set to `None` no checks are performed.
	///
	/// Incorporated as part of the Spurious Dragon upgrade via [EIP-155].
	///
	/// [EIP-155] https://eips.ethereum.org/EIPS/eip-155
	ChainId uint64 `json:"chain_id"`

	/// A list of addresses and storage keys that the transaction plans to access.
	///
	/// Added in [EIP-2930].
	///
	/// [EIP-2930] https://eips.ethereum.org/EIPS/eip-2930
	// access_list Vec<AccessListItem>

	/// The priority fee per gas.
	///
	/// Incorporated as part of the London upgrade via [EIP-1559].
	///
	/// [EIP-1559] https://eips.ethereum.org/EIPS/eip-1559
	// 	optinal
	GasPriorityFee U256 `json:"gas_priority_fee"`

	/// The list of blob versioned hashes. Per EIP there should be at least
	/// one blob present if [`Self::max_fee_per_blob_gas`] is `Some`.
	///
	/// Incorporated as part of the Cancun upgrade via [EIP-4844].
	///
	/// [EIP-4844] https://eips.ethereum.org/EIPS/eip-4844
	// blob_hashes Vec<B256>

	/// The max fee per blob gas.
	///
	/// Incorporated as part of the Cancun upgrade via [EIP-4844].
	///
	/// [EIP-4844] https://eips.ethereum.org/EIPS/eip-4844
	// max_fee_per_blob_gas Option<U256>

	/// List of authorizations that contains the signature that authorizes this
	/// caller to place the code to signer account.
	///
	/// Set EOA account code for one transaction
	///
	/// [EIP-Set EOA account code for one transaction](https://eips.ethereum.org/EIPS/eip-7702)
	// authorization_list Option<AuthorizationList>
}

type TransactionData = []byte
type txWithoutData struct {
	Caller         AccountAddress `json:"caller"`
	GasLimit       uint64         `json:"gas_limit"`
	GasPrice       U256           `json:"gas_price"`
	TransactTo     AccountAddress `json:"transact_to"`
	Value          U256           `json:"value"`
	Nonce          uint64         `json:"nonce"`
	ChainId        uint64         `json:"chain_id"`
	GasPriorityFee U256           `json:"gas_priority_fee"`
}

func (tx Transaction) Marshal() ([]byte, []byte) {
	txWithoutData := txWithoutData{
		Caller:         tx.Caller,
		GasLimit:       tx.GasLimit,
		GasPrice:       tx.GasPrice,
		TransactTo:     tx.TransactTo,
		Value:          tx.Value,
		Nonce:          tx.Nonce,
		ChainId:        tx.ChainId,
		GasPriorityFee: tx.GasPriorityFee,
	}

	jsonDataWithoutData, err := json.Marshal(txWithoutData)
	if err != nil {
		log.Fatalf("Failed to marshal transaction without data: %v", err)
	}

	return jsonDataWithoutData, tx.Data
}

func (block Block) Marshal() []byte {
	jsonData, err := json.Marshal(block)
	if err != nil {
		log.Fatalf("Failed to marshal block: %v", err)
	}
	return jsonData
}
