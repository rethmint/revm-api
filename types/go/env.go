package types

import (
	"encoding/hex"
	"errors"
	"math/big"
	"strings"
)

type AccountAddress [20]uint8

func NewAccountAddress(s string) (AccountAddress, error) {
	if !strings.HasPrefix(s, "0x") {
		return AccountAddress{}, errors.New("address must start with 0x")
	}
	if len(s) != 42 {
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

type U256 [32]byte

func NewU256(b *big.Int) U256 {
	var u U256
	bytes := b.Bytes()
	if len(bytes) > 32 {
		panic("input exceeds 32 bytes")
	}
	copy(u[32-len(bytes):], bytes)
	return u
}

func (u U256) String() string {
	return "0x" + hex.EncodeToString(u[:])
}

type BlockEnv struct {
	/// The number of ancestor blocks of this block (block height).
	Number U256
	/// Coinbase or miner or address that created and signed the block.
	///
	/// This is the receiver address of all the gas spent in the block.
	Coinbase AccountAddress

	/// The timestamp of the block in seconds since the UNIX epoch.
	Timestamp U256
	/// The gas limit of the block.
	GasLimit U256
	/// The base fee per gas added in the London upgrade with [EIP-1559].
	///
	/// [EIP-1559]: https://eips.ethereum.org/EIPS/eip-1559
	Basefee U256
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

// address =>  []storageKey
type AccessList map[AccountAddress][]U256

type TransactionEnv struct {
	/// Caller aka Author aka transaction signer.
	Caller AccountAddress
	/// The gas limit of the transaction.
	GasLimit uint64
	/// The gas price of the transaction.
	GasPrice U256
	/// The destination of the transaction.
	TransactTo AccountAddress
	/// The value sent to `transact_to`.
	Value U256

	Data []byte
	/// The nonce of the transaction.
	Nonce uint64

	/// The chain ID of the transaction. If set to `None` no checks are performed.
	///
	/// Incorporated as part of the Spurious Dragon upgrade via [EIP-155].
	///
	/// [EIP-155] https://eips.ethereum.org/EIPS/eip-155
	ChainId uint64

	/// A list of addresses and storage keys that the transaction plans to access.
	///
	/// Added in [EIP-2930].
	///
	/// [EIP-2930] https://eips.ethereum.org/EIPS/eip-2930
	AccessList AccessList

	/// The priority fee per gas.
	///
	/// Incorporated as part of the London upgrade via [EIP-1559].
	///
	/// [EIP-1559] https://eips.ethereum.org/EIPS/eip-1559
	// 	optinal
	GasPriorityFee U256

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
