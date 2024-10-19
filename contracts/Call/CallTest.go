// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package Call

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

// CallMetaData contains all meta data concerning the Call contract.
var CallMetaData = &bind.MetaData{
	ABI: "[{\"anonymous\":false,\"inputs\":[],\"name\":\"Advance\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"advance\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"mark\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"reflect\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
	Bin: "0x6080604052348015600e575f5ffd5b506101848061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c806354c5f3c5146100435780638c0d0c2914610061578063ea105ac71461007f575b5f5ffd5b61004b610089565b6040516100589190610104565b60405180910390f35b610069610090565b6040516100769190610135565b60405180910390f35b610087610097565b005b5f33905090565b5f43905090565b7f5a62da92468caa3ce06936d501af7dd9feec7359df68427efb69664a4486b67d60405160405180910390a1565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6100ee826100c5565b9050919050565b6100fe816100e4565b82525050565b5f6020820190506101175f8301846100f5565b92915050565b5f819050919050565b61012f8161011d565b82525050565b5f6020820190506101485f830184610126565b9291505056fea2646970667358221220094c9e66365cd24377cf4bc7e47a7e1fc6e7ea141d6bfc1986bce32202c4ef2e64736f6c634300081b0033",
}

// CallABI is the input ABI used to generate the binding from.
// Deprecated: Use CallMetaData.ABI instead.
var CallABI = CallMetaData.ABI

// CallBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use CallMetaData.Bin instead.
var CallBin = CallMetaData.Bin

// DeployCall deploys a new Ethereum contract, binding an instance of Call to it.
func DeployCall(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *Call, error) {
	parsed, err := CallMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(CallBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &Call{CallCaller: CallCaller{contract: contract}, CallTransactor: CallTransactor{contract: contract}, CallFilterer: CallFilterer{contract: contract}}, nil
}

// Call is an auto generated Go binding around an Ethereum contract.
type Call struct {
	CallCaller     // Read-only binding to the contract
	CallTransactor // Write-only binding to the contract
	CallFilterer   // Log filterer for contract events
}

// CallCaller is an auto generated read-only Go binding around an Ethereum contract.
type CallCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CallTransactor is an auto generated write-only Go binding around an Ethereum contract.
type CallTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CallFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type CallFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// CallSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type CallSession struct {
	Contract     *Call             // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CallCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type CallCallerSession struct {
	Contract *CallCaller   // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts // Call options to use throughout this session
}

// CallTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type CallTransactorSession struct {
	Contract     *CallTransactor   // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// CallRaw is an auto generated low-level Go binding around an Ethereum contract.
type CallRaw struct {
	Contract *Call // Generic contract binding to access the raw methods on
}

// CallCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type CallCallerRaw struct {
	Contract *CallCaller // Generic read-only contract binding to access the raw methods on
}

// CallTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type CallTransactorRaw struct {
	Contract *CallTransactor // Generic write-only contract binding to access the raw methods on
}

// NewCall creates a new instance of Call, bound to a specific deployed contract.
func NewCall(address common.Address, backend bind.ContractBackend) (*Call, error) {
	contract, err := bindCall(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &Call{CallCaller: CallCaller{contract: contract}, CallTransactor: CallTransactor{contract: contract}, CallFilterer: CallFilterer{contract: contract}}, nil
}

// NewCallCaller creates a new read-only instance of Call, bound to a specific deployed contract.
func NewCallCaller(address common.Address, caller bind.ContractCaller) (*CallCaller, error) {
	contract, err := bindCall(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &CallCaller{contract: contract}, nil
}

// NewCallTransactor creates a new write-only instance of Call, bound to a specific deployed contract.
func NewCallTransactor(address common.Address, transactor bind.ContractTransactor) (*CallTransactor, error) {
	contract, err := bindCall(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &CallTransactor{contract: contract}, nil
}

// NewCallFilterer creates a new log filterer instance of Call, bound to a specific deployed contract.
func NewCallFilterer(address common.Address, filterer bind.ContractFilterer) (*CallFilterer, error) {
	contract, err := bindCall(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &CallFilterer{contract: contract}, nil
}

// bindCall binds a generic wrapper to an already deployed contract.
func bindCall(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := CallMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Call *CallRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Call.Contract.CallCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Call *CallRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Call.Contract.CallTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Call *CallRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Call.Contract.CallTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_Call *CallCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _Call.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_Call *CallTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Call.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_Call *CallTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _Call.Contract.contract.Transact(opts, method, params...)
}

// Mark is a free data retrieval call binding the contract method 0x8c0d0c29.
//
// Solidity: function mark() view returns(uint256)
func (_Call *CallCaller) Mark(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _Call.contract.Call(opts, &out, "mark")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Mark is a free data retrieval call binding the contract method 0x8c0d0c29.
//
// Solidity: function mark() view returns(uint256)
func (_Call *CallSession) Mark() (*big.Int, error) {
	return _Call.Contract.Mark(&_Call.CallOpts)
}

// Mark is a free data retrieval call binding the contract method 0x8c0d0c29.
//
// Solidity: function mark() view returns(uint256)
func (_Call *CallCallerSession) Mark() (*big.Int, error) {
	return _Call.Contract.Mark(&_Call.CallOpts)
}

// Reflect is a free data retrieval call binding the contract method 0x54c5f3c5.
//
// Solidity: function reflect() view returns(address)
func (_Call *CallCaller) Reflect(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _Call.contract.Call(opts, &out, "reflect")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Reflect is a free data retrieval call binding the contract method 0x54c5f3c5.
//
// Solidity: function reflect() view returns(address)
func (_Call *CallSession) Reflect() (common.Address, error) {
	return _Call.Contract.Reflect(&_Call.CallOpts)
}

// Reflect is a free data retrieval call binding the contract method 0x54c5f3c5.
//
// Solidity: function reflect() view returns(address)
func (_Call *CallCallerSession) Reflect() (common.Address, error) {
	return _Call.Contract.Reflect(&_Call.CallOpts)
}

// Advance is a paid mutator transaction binding the contract method 0xea105ac7.
//
// Solidity: function advance() returns()
func (_Call *CallTransactor) Advance(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _Call.contract.Transact(opts, "advance")
}

// Advance is a paid mutator transaction binding the contract method 0xea105ac7.
//
// Solidity: function advance() returns()
func (_Call *CallSession) Advance() (*types.Transaction, error) {
	return _Call.Contract.Advance(&_Call.TransactOpts)
}

// Advance is a paid mutator transaction binding the contract method 0xea105ac7.
//
// Solidity: function advance() returns()
func (_Call *CallTransactorSession) Advance() (*types.Transaction, error) {
	return _Call.Contract.Advance(&_Call.TransactOpts)
}

// CallAdvanceIterator is returned from FilterAdvance and is used to iterate over the raw logs and unpacked data for Advance events raised by the Call contract.
type CallAdvanceIterator struct {
	Event *CallAdvance // Event containing the contract specifics and raw log

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
func (it *CallAdvanceIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(CallAdvance)
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
		it.Event = new(CallAdvance)
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
func (it *CallAdvanceIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *CallAdvanceIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// CallAdvance represents a Advance event raised by the Call contract.
type CallAdvance struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterAdvance is a free log retrieval operation binding the contract event 0x5a62da92468caa3ce06936d501af7dd9feec7359df68427efb69664a4486b67d.
//
// Solidity: event Advance()
func (_Call *CallFilterer) FilterAdvance(opts *bind.FilterOpts) (*CallAdvanceIterator, error) {

	logs, sub, err := _Call.contract.FilterLogs(opts, "Advance")
	if err != nil {
		return nil, err
	}
	return &CallAdvanceIterator{contract: _Call.contract, event: "Advance", logs: logs, sub: sub}, nil
}

// WatchAdvance is a free log subscription operation binding the contract event 0x5a62da92468caa3ce06936d501af7dd9feec7359df68427efb69664a4486b67d.
//
// Solidity: event Advance()
func (_Call *CallFilterer) WatchAdvance(opts *bind.WatchOpts, sink chan<- *CallAdvance) (event.Subscription, error) {

	logs, sub, err := _Call.contract.WatchLogs(opts, "Advance")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(CallAdvance)
				if err := _Call.contract.UnpackLog(event, "Advance", log); err != nil {
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

// ParseAdvance is a log parse operation binding the contract event 0x5a62da92468caa3ce06936d501af7dd9feec7359df68427efb69664a4486b67d.
//
// Solidity: event Advance()
func (_Call *CallFilterer) ParseAdvance(log types.Log) (*CallAdvance, error) {
	event := new(CallAdvance)
	if err := _Call.contract.UnpackLog(event, "Advance", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}