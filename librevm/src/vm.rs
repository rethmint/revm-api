use revm_primitives::{ Block, Transaction };

use crate::ByteSliceView;

pub(crate) fn set_env(
    block: ByteSliceView,
    tx: ByteSliceView
) -> Env<BlockT: Block, TxT: Transaction> {
    let cfg = CfgEnv::default();
    cfg.chain_id = chain_id;
    let mut env: Env<BlockT: Block, TxT: Transaction> = Env::default();
    env.cfg = cfg;
    env.block = block;
    env.tx = tx;

    env
}

pub(crate) fn set_vm();
