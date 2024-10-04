package revm

import (
	"github.com/rethmint/revm-api/api"
	"github.com/rethmint/revm-api/types"
)

// VM struct is the core of initiavm.
type VM struct {
	inner api.VM
}

// NewVm return VM instance
// handler
func NewVM() VM {
	inner := api.InitVM()
	return VM{inner}
}

func (vm *VM) Destroy() {
	api.ReleaseVM(vm.inner)
}

// ExecuteTx execute transaction based on revm
// for bootstrapping genesis
func (vm *VM) ExecuteTx(
	kvStore api.KVStore,
	block types.Block,
	tx types.Transaction,
	data []byte,
) (types.ResultData, error) {
	res, err := api.ExecuteTx(
		vm.inner,
		kvStore,
		block,
		tx,
		data,
	)
	if err != nil {
		return nil, err
	}
	resultData, err := res.ResultData()
	if err != nil {
		return nil, err
	}
	return resultData, nil
}

func (vm *VM) Query(
	kvStore api.KVStore,
	tx types.Transaction,
	block types.Block,
	data []byte,
) (types.ResultData, error) {
	res, err := api.Query(
		vm.inner,
		kvStore,
		block,
		tx,
		data,
	)
	if err != nil {
		return nil, err
	}
	resultData, err := res.ResultData()
	if err != nil {
		return nil, err
	}
	return resultData, nil
}
