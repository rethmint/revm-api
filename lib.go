package revm_api

import (
	"github.com/rethmint/revm-api/api"
	types "github.com/rethmint/revm-api/types/go"
)

// VM struct is the core of initiavm.
type VM struct {
	Inner api.VM
}

type Cron struct {
	Inner api.Cron
}

func NewCron() Cron {
	inner := api.InitCron()
	return Cron{inner}
}

func (cron *Cron) Destroy() {
	api.ReleaseCron(cron.Inner)
}

// NewVm return VM instance
// handler
func NewVM(SPEC_ID uint8) VM {
	inner := api.InitVM(SPEC_ID)
	return VM{inner}
}

func (vm *VM) Destroy() {
	api.ReleaseVM(vm.Inner)
}

// ExecuteTx execute transaction based on revm
// for bootstrapping genesis
func (vm *VM) ExecuteTx(
	kvStore api.KVStore,
	block types.SerializedBlock,
	tx types.SerializedTransaction,
) (types.ExecutionResult, error) {
	res, err := api.ExecuteTx(
		vm.Inner,
		kvStore,
		&block,
		&tx,
	)
	if err != nil {
		return nil, err
	}

	return res, nil
}

func (vm *VM) QueryTx(
	kvStore api.KVStore,
	block types.SerializedBlock,
	tx types.SerializedTransaction,
) (types.ExecutionResult, error) {
	res, err := api.QueryTx(
		vm.Inner,
		kvStore,
		&block,
		&tx,
	)
	if err != nil {
		return nil, err
	}

	return res, nil
}
