// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package fibonacci

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// FibonacciMetaData contains all meta data concerning the Fibonacci contract.
var FibonacciMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"number\",\"type\":\"uint256\"}],\"name\":\"fibonacci\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
	Bin: "0x6080604052348015600e575f5ffd5b5061010c8061001c5f395ff3fe6080604052348015600e575f5ffd5b50600436106026575f3560e01c806361047ff414602a575b5f5ffd5b60396035366004608c565b604b565b60405190815260200160405180910390f35b5f815f03605957505f919050565b81600103606857506001919050565b6073603560028460b6565b607e603560018560b6565b6086919060c6565b92915050565b5f60208284031215609b575f5ffd5b5035919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115608657608660a2565b80820180821115608657608660a256fea264697066735822122075f2b7835a429cbebf20df7d12b06806472a0714419b9b29eac058b3a7f6d80c64736f6c634300081b0033",
}

// FibonacciABI is the input ABI used to generate the binding from.
// Deprecated: Use FibonacciMetaData.ABI instead.
var FibonacciABI = FibonacciMetaData.ABI

// FibonacciBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use FibonacciMetaData.Bin instead.
var FibonacciBin = FibonacciMetaData.Bin

// DeployFibonacci deploys a new Ethereum contract, binding an instance of Fibonacci to it.
func DeployFibonacci(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *Fibonacci, error) {
	parsed, err := FibonacciMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(FibonacciBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &Fibonacci{FibonacciCaller: FibonacciCaller{contract: contract}, FibonacciTransactor: FibonacciTransactor{contract: contract}, FibonacciFilterer: FibonacciFilterer{contract: contract}}, nil
}

// Fibonacci is an auto generated Go binding around an Ethereum contract.
type Fibonacci struct {
	FibonacciCaller     // Read-only binding to the contract
	FibonacciTransactor // Write-only binding to the contract
	FibonacciFilterer   // Log filterer for contract events
}

// FibonacciCaller is an auto generated read-only Go binding around an Ethereum contract.
type FibonacciCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FibonacciTransactor is an auto generated write-only Go binding around an Ethereum contract.
type FibonacciTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FibonacciFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type FibonacciFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FibonacciSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type FibonacciSession struct {
	Contract     *Fibonacci        // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// FibonacciCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type FibonacciCallerSession struct {
	Contract *FibonacciCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts    // Call options to use throughout this session
}

// FibonacciTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type FibonacciTransactorSession struct {
	Contract     *FibonacciTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts    // Transaction auth options to use throughout this session
}

// FibonacciRaw is an auto generated low-level Go binding around an Ethereum contract.
type FibonacciRaw struct {
	Contract *Fibonacci // Generic contract binding to access the raw methods on
}

// FibonacciCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type FibonacciCallerRaw struct {
	Contract *FibonacciCaller // Generic read-only contract binding to access the raw methods on
}

// FibonacciTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type FibonacciTransactorRaw struct {
	Contract *FibonacciTransactor // Generic write-only contract binding to access the raw methods on
}

// NewFibonacci creates a new instance of Fibonacci, bound to a specific deployed contract.
func NewFibonacci(address common.Address, backend bind.ContractBackend) (*Fibonacci, error) {
	contract, err := bindFibonacci(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Fibonacci{FibonacciCaller: FibonacciCaller{contract: contract}, FibonacciTransactor: FibonacciTransactor{contract: contract}, FibonacciFilterer: FibonacciFilterer{contract: contract}}, nil
}

// NewFibonacciCaller creates a new read-only instance of Fibonacci, bound to a specific deployed contract.
func NewFibonacciCaller(address common.Address, caller bind.ContractCaller) (*FibonacciCaller, error) {
	contract, err := bindFibonacci(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &FibonacciCaller{contract: contract}, nil
}

// NewFibonacciTransactor creates a new write-only instance of Fibonacci, bound to a specific deployed contract.
func NewFibonacciTransactor(address common.Address, transactor bind.ContractTransactor) (*FibonacciTransactor, error) {
	contract, err := bindFibonacci(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &FibonacciTransactor{contract: contract}, nil
}

// NewFibonacciFilterer creates a new log filterer instance of Fibonacci, bound to a specific deployed contract.
func NewFibonacciFilterer(address common.Address, filterer bind.ContractFilterer) (*FibonacciFilterer, error) {
	contract, err := bindFibonacci(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &FibonacciFilterer{contract: contract}, nil
}

// bindFibonacci binds a generic wrapper to an already deployed contract.
func bindFibonacci(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := FibonacciMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Fibonacci *FibonacciRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Fibonacci.Contract.FibonacciCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Fibonacci *FibonacciRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Fibonacci.Contract.FibonacciTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Fibonacci *FibonacciRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Fibonacci.Contract.FibonacciTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Fibonacci *FibonacciCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Fibonacci.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Fibonacci *FibonacciTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Fibonacci.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Fibonacci *FibonacciTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Fibonacci.Contract.contract.Transact(opts, method, params...)
}

// Fibonacci is a paid mutator transaction binding the contract method 0x61047ff4.
//
// Solidity: function fibonacci(uint256 number) returns(uint256 result)
func (_Fibonacci *FibonacciTransactor) Fibonacci(opts *bind.TransactOpts, number *big.Int) (*types.Transaction, error) {
	return _Fibonacci.contract.Transact(opts, "fibonacci", number)
}

// Fibonacci is a paid mutator transaction binding the contract method 0x61047ff4.
//
// Solidity: function fibonacci(uint256 number) returns(uint256 result)
func (_Fibonacci *FibonacciSession) Fibonacci(number *big.Int) (*types.Transaction, error) {
	return _Fibonacci.Contract.Fibonacci(&_Fibonacci.TransactOpts, number)
}

// Fibonacci is a paid mutator transaction binding the contract method 0x61047ff4.
//
// Solidity: function fibonacci(uint256 number) returns(uint256 result)
func (_Fibonacci *FibonacciTransactorSession) Fibonacci(number *big.Int) (*types.Transaction, error) {
	return _Fibonacci.Contract.Fibonacci(&_Fibonacci.TransactOpts, number)
}
