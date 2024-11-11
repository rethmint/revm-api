// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package Test

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

// TestMetaData contains all meta data concerning the Test contract.
var TestMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"oldCount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newCount\",\"type\":\"uint256\"}],\"name\":\"increased\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"count\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"n\",\"type\":\"uint64\"}],\"name\":\"get_blockhash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"get_msg_sender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"increase\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"}]",
	Bin: "0x6080604052610385806100115f395ff3fe60806040526004361061003e575f3560e01c806306661abd14610042578063ac7fde5f1461006c578063cfcc3abe146100a8578063e8927fbc146100d2575b5f80fd5b34801561004d575f80fd5b506100566100dc565b6040516100639190610173565b60405180910390f35b348015610077575f80fd5b50610092600480360381019061008d91906101cd565b6100e1565b60405161009f9190610210565b60405180910390f35b3480156100b3575f80fd5b506100bc6100f5565b6040516100c99190610268565b60405180910390f35b6100da6100fc565b005b5f5481565b5f8167ffffffffffffffff16409050919050565b5f33905090565b5f8081548092919061010d906102ae565b91905055507f61996fe196f72cb598c483e896a1221263a28bb630480aa89495f737d4a8e3df60015f5461014191906102f5565b5f54604051610151929190610328565b60405180910390a1565b5f819050919050565b61016d8161015b565b82525050565b5f6020820190506101865f830184610164565b92915050565b5f80fd5b5f67ffffffffffffffff82169050919050565b6101ac81610190565b81146101b6575f80fd5b50565b5f813590506101c7816101a3565b92915050565b5f602082840312156101e2576101e161018c565b5b5f6101ef848285016101b9565b91505092915050565b5f819050919050565b61020a816101f8565b82525050565b5f6020820190506102235f830184610201565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61025282610229565b9050919050565b61026281610248565b82525050565b5f60208201905061027b5f830184610259565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6102b88261015b565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036102ea576102e9610281565b5b600182019050919050565b5f6102ff8261015b565b915061030a8361015b565b925082820390508181111561032257610321610281565b5b92915050565b5f60408201905061033b5f830185610164565b6103486020830184610164565b939250505056fea2646970667358221220c07b6902dac89c1059e217782116d585a01b2fa0ef6d890c01ca5bdde24fcd5f64736f6c63430008190033",
}

// TestABI is the input ABI used to generate the binding from.
// Deprecated: Use TestMetaData.ABI instead.
var TestABI = TestMetaData.ABI

// TestBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use TestMetaData.Bin instead.
var TestBin = TestMetaData.Bin

// DeployTest deploys a new Ethereum contract, binding an instance of Test to it.
func DeployTest(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *Test, error) {
	parsed, err := TestMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(TestBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &Test{TestCaller: TestCaller{contract: contract}, TestTransactor: TestTransactor{contract: contract}, TestFilterer: TestFilterer{contract: contract}}, nil
}

// Test is an auto generated Go binding around an Ethereum contract.
type Test struct {
	TestCaller     // Read-only binding to the contract
	TestTransactor // Write-only binding to the contract
	TestFilterer   // Log filterer for contract events
}

// TestCaller is an auto generated read-only Go binding around an Ethereum contract.
type TestCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// TestTransactor is an auto generated write-only Go binding around an Ethereum contract.
type TestTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// TestFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type TestFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// TestSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type TestSession struct {
	Contract     *Test             // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// TestCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type TestCallerSession struct {
	Contract *TestCaller   // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts // Call options to use throughout this session
}

// TestTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type TestTransactorSession struct {
	Contract     *TestTransactor   // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// TestRaw is an auto generated low-level Go binding around an Ethereum contract.
type TestRaw struct {
	Contract *Test // Generic contract binding to access the raw methods on
}

// TestCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type TestCallerRaw struct {
	Contract *TestCaller // Generic read-only contract binding to access the raw methods on
}

// TestTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type TestTransactorRaw struct {
	Contract *TestTransactor // Generic write-only contract binding to access the raw methods on
}

// NewTest creates a new instance of Test, bound to a specific deployed contract.
func NewTest(address common.Address, backend bind.ContractBackend) (*Test, error) {
	contract, err := bindTest(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Test{TestCaller: TestCaller{contract: contract}, TestTransactor: TestTransactor{contract: contract}, TestFilterer: TestFilterer{contract: contract}}, nil
}

// NewTestCaller creates a new read-only instance of Test, bound to a specific deployed contract.
func NewTestCaller(address common.Address, caller bind.ContractCaller) (*TestCaller, error) {
	contract, err := bindTest(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &TestCaller{contract: contract}, nil
}

// NewTestTransactor creates a new write-only instance of Test, bound to a specific deployed contract.
func NewTestTransactor(address common.Address, transactor bind.ContractTransactor) (*TestTransactor, error) {
	contract, err := bindTest(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &TestTransactor{contract: contract}, nil
}

// NewTestFilterer creates a new log filterer instance of Test, bound to a specific deployed contract.
func NewTestFilterer(address common.Address, filterer bind.ContractFilterer) (*TestFilterer, error) {
	contract, err := bindTest(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &TestFilterer{contract: contract}, nil
}

// bindTest binds a generic wrapper to an already deployed contract.
func bindTest(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := TestMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Test *TestRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Test.Contract.TestCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Test *TestRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Test.Contract.TestTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Test *TestRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Test.Contract.TestTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Test *TestCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Test.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Test *TestTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Test.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Test *TestTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Test.Contract.contract.Transact(opts, method, params...)
}

// Count is a free data retrieval call binding the contract method 0x06661abd.
//
// Solidity: function count() view returns(uint256)
func (_Test *TestCaller) Count(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _Test.contract.Call(opts, &out, "count")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Count is a free data retrieval call binding the contract method 0x06661abd.
//
// Solidity: function count() view returns(uint256)
func (_Test *TestSession) Count() (*big.Int, error) {
	return _Test.Contract.Count(&_Test.CallOpts)
}

// Count is a free data retrieval call binding the contract method 0x06661abd.
//
// Solidity: function count() view returns(uint256)
func (_Test *TestCallerSession) Count() (*big.Int, error) {
	return _Test.Contract.Count(&_Test.CallOpts)
}

// GetBlockhash is a free data retrieval call binding the contract method 0xac7fde5f.
//
// Solidity: function get_blockhash(uint64 n) view returns(bytes32)
func (_Test *TestCaller) GetBlockhash(opts *bind.CallOpts, n uint64) ([32]byte, error) {
	var out []interface{}
	err := _Test.contract.Call(opts, &out, "get_blockhash", n)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// GetBlockhash is a free data retrieval call binding the contract method 0xac7fde5f.
//
// Solidity: function get_blockhash(uint64 n) view returns(bytes32)
func (_Test *TestSession) GetBlockhash(n uint64) ([32]byte, error) {
	return _Test.Contract.GetBlockhash(&_Test.CallOpts, n)
}

// GetBlockhash is a free data retrieval call binding the contract method 0xac7fde5f.
//
// Solidity: function get_blockhash(uint64 n) view returns(bytes32)
func (_Test *TestCallerSession) GetBlockhash(n uint64) ([32]byte, error) {
	return _Test.Contract.GetBlockhash(&_Test.CallOpts, n)
}

// GetMsgSender is a free data retrieval call binding the contract method 0xcfcc3abe.
//
// Solidity: function get_msg_sender() view returns(address)
func (_Test *TestCaller) GetMsgSender(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Test.contract.Call(opts, &out, "get_msg_sender")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// GetMsgSender is a free data retrieval call binding the contract method 0xcfcc3abe.
//
// Solidity: function get_msg_sender() view returns(address)
func (_Test *TestSession) GetMsgSender() (common.Address, error) {
	return _Test.Contract.GetMsgSender(&_Test.CallOpts)
}

// GetMsgSender is a free data retrieval call binding the contract method 0xcfcc3abe.
//
// Solidity: function get_msg_sender() view returns(address)
func (_Test *TestCallerSession) GetMsgSender() (common.Address, error) {
	return _Test.Contract.GetMsgSender(&_Test.CallOpts)
}

// Increase is a paid mutator transaction binding the contract method 0xe8927fbc.
//
// Solidity: function increase() payable returns()
func (_Test *TestTransactor) Increase(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Test.contract.Transact(opts, "increase")
}

// Increase is a paid mutator transaction binding the contract method 0xe8927fbc.
//
// Solidity: function increase() payable returns()
func (_Test *TestSession) Increase() (*types.Transaction, error) {
	return _Test.Contract.Increase(&_Test.TransactOpts)
}

// Increase is a paid mutator transaction binding the contract method 0xe8927fbc.
//
// Solidity: function increase() payable returns()
func (_Test *TestTransactorSession) Increase() (*types.Transaction, error) {
	return _Test.Contract.Increase(&_Test.TransactOpts)
}

// TestIncreasedIterator is returned from FilterIncreased and is used to iterate over the raw logs and unpacked data for Increased events raised by the Test contract.
type TestIncreasedIterator struct {
	Event *TestIncreased // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *TestIncreasedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(TestIncreased)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(TestIncreased)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *TestIncreasedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *TestIncreasedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// TestIncreased represents a Increased event raised by the Test contract.
type TestIncreased struct {
	OldCount *big.Int
	NewCount *big.Int
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterIncreased is a free log retrieval operation binding the contract event 0x61996fe196f72cb598c483e896a1221263a28bb630480aa89495f737d4a8e3df.
//
// Solidity: event increased(uint256 oldCount, uint256 newCount)
func (_Test *TestFilterer) FilterIncreased(opts *bind.FilterOpts) (*TestIncreasedIterator, error) {

	logs, sub, err := _Test.contract.FilterLogs(opts, "increased")
	if err != nil {
		return nil, err
	}
	return &TestIncreasedIterator{contract: _Test.contract, event: "increased", logs: logs, sub: sub}, nil
}

// WatchIncreased is a free log subscription operation binding the contract event 0x61996fe196f72cb598c483e896a1221263a28bb630480aa89495f737d4a8e3df.
//
// Solidity: event increased(uint256 oldCount, uint256 newCount)
func (_Test *TestFilterer) WatchIncreased(opts *bind.WatchOpts, sink chan<- *TestIncreased) (event.Subscription, error) {

	logs, sub, err := _Test.contract.WatchLogs(opts, "increased")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(TestIncreased)
				if err := _Test.contract.UnpackLog(event, "increased", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseIncreased is a log parse operation binding the contract event 0x61996fe196f72cb598c483e896a1221263a28bb630480aa89495f737d4a8e3df.
//
// Solidity: event increased(uint256 oldCount, uint256 newCount)
func (_Test *TestFilterer) ParseIncreased(log types.Log) (*TestIncreased, error) {
	event := new(TestIncreased)
	if err := _Test.contract.UnpackLog(event, "increased", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
