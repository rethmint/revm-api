mod cfg;
mod cron;
mod leveldb;

use alloy_primitives::{address, U256};
use color_eyre::Result;
use revm::primitives::{Env, SpecId, TransactTo};
use revmc::{eyre::ensure, EvmCompiler, EvmLlvmBackend};
use std::{path::PathBuf, str::FromStr};

pub use cfg::*;
pub use cron::*;
pub use leveldb::*;

pub struct RuntimeJit {
    pub unit: JitUnit,
    pub cfg: JitCfg,
}

#[derive(Clone, Debug, Default)]
pub struct JitUnit {
    pub name: &'static str,
    pub bytecode: Vec<u8>,
    pub calldata: Vec<u8>,
    pub stack_input: Vec<U256>,
}

impl JitUnit {
    pub fn new(name: &'static str, bytecode: Vec<u8>, stack_input_size: u64) -> Self {
        Self {
            name,
            bytecode,
            calldata: Vec::new(),
            stack_input: vec![U256::from(stack_input_size)],
        }
    }
}

impl RuntimeJit {
    pub fn new(unit: JitUnit, cfg: JitCfg) -> Self {
        Self { unit, cfg }
    }

    pub fn compile(&self) -> Result<()> {
        let _ = color_eyre::install();

        // Build the compiler.
        let context = revmc::llvm::inkwell::context::Context::create();
        let target = revmc::Target::new(
            self.cfg.target,
            self.cfg.target_cpu.clone(),
            self.cfg.target_features.clone(),
        );
        let backend =
            EvmLlvmBackend::new_for_target(&context, self.cfg.aot, self.cfg.opt_level, &target)?;
        let mut compiler = EvmCompiler::new(backend);
        let out_pathbuf = PathBuf::from_str(self.cfg.out_dir)?;
        compiler.set_dump_to(Some(out_pathbuf));
        compiler.gas_metering(self.cfg.no_gas);
        unsafe { compiler.stack_bound_checks(self.cfg.no_len_checks) };
        compiler.frame_pointers(true);
        compiler.debug_assertions(self.cfg.debug_assertions);
        compiler.validate_eof(self.cfg.no_validate);

        compiler.set_module_name(self.unit.name);

        let calldata = if let Some(calldata) = &self.cfg.calldata {
            revmc::primitives::hex::decode(calldata)?.into()
        } else {
            self.unit.calldata.clone().into()
        };
        let gas_limit = self.cfg.gas_limit;

        let mut env = Env::default();
        env.tx.caller = address!("0000000000000000000000000000000000000001");
        env.tx.transact_to = TransactTo::Call(address!("0000000000000000000000000000000000000002"));
        env.tx.data = calldata;
        env.tx.gas_limit = gas_limit;

        let bytecode =
            revm::interpreter::analysis::to_analysed(revm::primitives::Bytecode::new_raw(
                revm::primitives::Bytes::copy_from_slice(&self.unit.bytecode),
            ));
        let contract = revm::interpreter::Contract::new_env(&env, bytecode, None);

        let bytecode = contract.bytecode.original_byte_slice();

        let spec_id = if self.cfg.eof {
            SpecId::OSAKA
        } else {
            self.cfg.spec_id.into()
        };
        if !self.unit.stack_input.is_empty() {
            compiler.inspect_stack_length(true);
        }

        let _f_id = compiler.translate(self.unit.name, bytecode, spec_id)?;

        if self.cfg.aot {
            let out_dir = if let Some(out_dir) = compiler.out_dir() {
                out_dir.join(self.unit.name)
            } else {
                let dir = std::env::temp_dir().join("revmc-cli").join(self.unit.name);
                std::fs::create_dir_all(&dir)?;
                dir
            };

            // Compile.
            let obj = out_dir.join("a.o");
            compiler.write_object_to_file(&obj)?;
            ensure!(obj.exists(), "Failed to write object file");
            eprintln!("Compiled object file to {}", obj.display());

            // Link.
            if !self.cfg.no_link {
                let so = out_dir.join("a.so");
                let linker = revmc::Linker::new();
                linker.link(&so, [obj.to_str().unwrap()])?;
                ensure!(so.exists(), "Failed to link object file");
                eprintln!("Linked shared object file to {}", so.display());
            }
        }

        Ok(())
    }
}
