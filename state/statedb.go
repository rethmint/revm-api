package state

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/rawdb"
	gethstate "github.com/ethereum/go-ethereum/core/state"
	"github.com/ethereum/go-ethereum/core/types"
)

// Wrap Database to support revm ffi call.
type StateDB struct {
	trie          gethstate.Trie
	cachedb       gethstate.CachingDB
	reader        gethstate.Reader
	blockHashFunc func(number uint64) common.Hash
}

// New creates a new state from a given trie.
func New(root common.Hash, cachedb gethstate.CachingDB) (*StateDB, error) {
	reader, err := cachedb.Reader(root)
	if err != nil {
		return nil, err
	}

	trie, err := cachedb.OpenTrie(root)
	if err != nil {
		return nil, err
	}

	return &StateDB{
		trie:          trie,
		cachedb:       cachedb,
		reader:        reader,
		blockHashFunc: nil,
	}, nil
}

/******Getter******/

func (state *StateDB) GetAccount(address common.Address) *types.StateAccount {
	account, err := state.reader.Account(address)

	if err != nil {
		panic("failed to get account: " + err.Error())
	}

	return account
}

func (state *StateDB) GetCodeByHash(codeHash common.Hash) []byte {
	code, err := state.reader.Code(common.Address{}, codeHash)
	if err != nil {
		panic("failed to get code: " + err.Error())
	}

	return code
}

func (state *StateDB) GetStorage(address common.Address, key common.Hash) common.Hash {
	storage, err := state.reader.Storage(address, key)
	if err != nil {
		panic("failed to get storage: " + err.Error())
	}

	return storage
}

func (state *StateDB) GetBlockHash(number uint64) common.Hash {
	// GetBlockHash should be set on evm instance creation
	if state.blockHashFunc == nil {
		panic("blockHashFunc is not set")
	}
	return state.blockHashFunc(number)
}

func (state *StateDB) SetBlockHashFunc(blockHashFunc func(number uint64) common.Hash) {
	state.blockHashFunc = blockHashFunc
}

/******Commit******/
func (state *StateDB) Commit(codes map[common.Hash][]byte, storages map[common.Address]map[common.Hash]common.Hash, accounts map[common.Address]*types.StateAccount, deleted []common.Address) (common.Hash, error) {
	// storages update on cachedb
	err := state.updateStorages(storages)
	if err != nil {
		return common.Hash{}, err
	}
	err = state.deleteStorages(deleted)
	if err != nil {
		return common.Hash{}, err
	}
	// accounts update on cachedb
	err = state.updateAccounts(accounts)
	if err != nil {
		return common.Hash{}, err
	}
	err = state.deleteAccounts(deleted)
	if err != nil {
		return common.Hash{}, err
	}
	// commit updated codes
	root, _ := state.trie.Commit(true)
	return root, nil
}

/******Code******/
// There is no interface for update in cachedb. so directly commit on rawdb
func (state *StateDB) commitUpdatedCode(codes map[common.Hash][]byte) error {
	if db := state.cachedb.TrieDB().Disk(); db != nil {
		batch := db.NewBatch()
		for codeHash, code := range codes {
			rawdb.WriteCode(batch, codeHash, code)
		}
		if err := batch.Write(); err != nil {
			return err
		}
	}
	return nil
}

/******Storages******/
func (state *StateDB) updateStorages(storages map[common.Address]map[common.Hash]common.Hash) error {
	for addr, kv := range storages {
		for key, value := range kv {
			err := state.trie.UpdateStorage(addr, key.Bytes(), value.Bytes())
			if err != nil {
				return err
			}
		}
	}
	return nil
}

func (state *StateDB) deleteStorages(addrs []common.Address) error {
	for _, addr := range addrs {
		err := state.trie.DeleteAccount(addr)
		if err != nil {
			return err
		}
	}
	return nil
}

/******Accounts******/
func (state *StateDB) updateAccounts(accounts map[common.Address]*types.StateAccount) error {
	for addr, account := range accounts {
		code := state.GetCodeByHash(common.Hash(account.CodeHash))
		err := state.trie.UpdateAccount(addr, account, len(code))
		if err != nil {
			return err
		}
	}
	return nil
}

func (state *StateDB) deleteAccounts(addrs []common.Address) error {
	for _, addr := range addrs {
		err := state.trie.DeleteAccount(addr)
		if err != nil {
			return err
		}
	}
	return nil
}
