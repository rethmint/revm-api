use revm_primitives::{Block, CfgEnv, Env, Transaction};

use crate::ByteSliceView;

pub(crate) fn set_env(
    block: ByteSliceView,
    tx: ByteSliceView,
) -> Env<BlockT: Block, TxT: Transaction> {
    let cfg = CfgEnv::default();
    cfg.chain_id = "0";
    let mut env: Env<BlockT: Block, TxT: Transaction> = Env::default();
    env.cfg = cfg;
    env.block = block;
    env.tx = tx;

    env
}

pub(crate) fn set_vm() {}
