mod cfg;
mod cron;
mod key;
mod sled;

use alloy_primitives::{address, hex::encode, U256};
use color_eyre::Result;
use revm::primitives::{Env, SpecId, TransactTo};
use revmc::{eyre::ensure, EvmCompiler, EvmLlvmBackend};
use std::{path::PathBuf, str::FromStr};

pub use cfg::*;
pub use cron::*;
pub use key::*;
pub use sled::*;

pub const JIT_OUT_PATH: &str = "librevm/out";

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
    pub fn new(
        name: &'static str,
        bytecode: Vec<u8>,
        stack_input_size: u64,
        calldata: Vec<u8>,
    ) -> Self {
        Self {
            name,
            bytecode,
            calldata,
            stack_input: vec![U256::from(stack_input_size)],
        }
    }
}

impl RuntimeJit {
    pub fn new(unit: JitUnit, cfg: JitCfg) -> Self {
        Self { unit, cfg }
    }

    pub fn compile(&self) -> Result<()> {
        println!("Starting compile function");
        let _ = color_eyre::install();
        println!("color_eyre installed");

        // Build the compiler.
        let context = revmc::llvm::inkwell::context::Context::create();
        println!("LLVM context created");

        let target = revmc::Target::new(
            self.cfg.target,
            self.cfg.target_cpu.clone(),
            self.cfg.target_features.clone(),
        );
        println!("Target created: {:?}", target);

        let backend =
            EvmLlvmBackend::new_for_target(&context, self.cfg.aot, self.cfg.opt_level, &target)?;
        println!("Backend created for target");

        let mut compiler = EvmCompiler::new(backend);
        println!("Compiler initialized");

        let out_pathbuf = PathBuf::from_str(self.cfg.out_dir)?;
        println!("Output path set to: {:?}", out_pathbuf);

        compiler.set_dump_to(Some(out_pathbuf));
        println!("Dump path set in compiler");

        compiler.gas_metering(self.cfg.no_gas);
        println!("Gas metering configured: {:?}", self.cfg.no_gas);

        unsafe { compiler.stack_bound_checks(self.cfg.no_len_checks) };
        println!("Stack bound checks set: {:?}", self.cfg.no_len_checks);

        compiler.frame_pointers(true);
        println!("Frame pointers enabled");

        compiler.debug_assertions(self.cfg.debug_assertions);
        println!(
            "Debug assertions configured: {:?}",
            self.cfg.debug_assertions
        );

        compiler.set_module_name(self.unit.name);
        println!("Module name set to: {:?}", self.unit.name);

        let calldata = if let Some(calldata) = &self.cfg.calldata {
            let decoded = revmc::primitives::hex::decode(calldata)?;
            println!("Decoded calldata from config: {:?}", decoded);
            decoded.into()
        } else {
            let cloned_calldata = self.unit.calldata.clone().into();
            println!("Using default calldata from unit: {:?}", cloned_calldata);
            cloned_calldata
        };
        println!("Calldata set: {:?}", calldata);

        let gas_limit = self.cfg.gas_limit;
        println!("Gas limit set to: {:?}", gas_limit);

        let mut env = Env::default();
        env.tx.caller = address!("0000000000000000000000000000000000000001");
        env.tx.transact_to = TransactTo::Call(address!("0000000000000000000000000000000000000002"));
        env.tx.data = calldata;
        env.tx.gas_limit = gas_limit;
        println!("Environment transaction data set up: {:?}", env.tx);

        let bytecode =
            revm::interpreter::analysis::to_analysed(revm::primitives::Bytecode::new_raw(
                revm::primitives::Bytes::copy_from_slice(&self.unit.bytecode),
            ));
        println!("Bytecode prepared for contract");

        let contract = revm::interpreter::Contract::new_env(&env, bytecode, None);
        println!("Contract created");

        let bytecode = contract.bytecode.original_byte_slice();
        let hex_format = format!("0x{}", encode(bytecode));
        println!("Original bytecode in hex: {}", hex_format);

        let spec_id = if self.cfg.eof {
            SpecId::OSAKA
        } else {
            self.cfg.spec_id.into()
        };
        println!("Spec ID selected: {:?}", spec_id);

        if !self.unit.stack_input.is_empty() {
            compiler.inspect_stack_length(true);
            println!("Stack inspection enabled due to non-empty stack input");
        }

        let _f_id = compiler.translate(self.unit.name, bytecode, spec_id)?;
        println!(
            "Compilation translation completed with function ID: {:?}",
            _f_id
        );

        if self.cfg.aot {
            println!("AOT compilation is enabled");

            let out_dir = if let Some(out_dir) = compiler.out_dir() {
                out_dir.join(&self.unit.name)
            } else {
                let dir = std::env::temp_dir()
                    .join(JIT_OUT_PATH)
                    .join(&self.unit.name);
                std::fs::create_dir_all(&dir)?;
                dir
            };
            println!("Output directory for object file: {:?}", out_dir);

            // Compile.
            let obj = out_dir.join("a.o");
            compiler.write_object_to_file(&obj)?;
            println!("Object file written to: {:?}", obj);
            ensure!(obj.exists(), "Failed to write object file");

            // Link.
            if !self.cfg.no_link {
                let so = out_dir.join("a.so");
                let linker = revmc::Linker::new();
                linker.link(&so, [obj.to_str().unwrap()])?;
                println!("Shared object file linked to: {:?}", so);
                ensure!(so.exists(), "Failed to link object file");
            }
        }

        println!("Compile function completed successfully");
        Ok(())
    }
}
