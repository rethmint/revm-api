mod cfg;
mod cron;
mod key;
mod sled;

use std::path::PathBuf;

use alloy_primitives::U256;
use color_eyre::Result;
use revm::primitives::SpecId;
use revmc::{eyre::ensure, EvmCompiler, EvmLlvmBackend};
use tempdir::TempDir;
use tokio::fs;

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
    pub stack_input: Vec<U256>,
}

impl JitUnit {
    pub fn new(name: &'static str, stack_input_size: u64) -> Self {
        Self {
            name,
            stack_input: vec![U256::from(stack_input_size)],
        }
    }
}

impl RuntimeJit {
    pub fn new(unit: JitUnit, cfg: JitCfg) -> Self {
        Self { unit, cfg }
    }

    pub async fn compile(&self, bytecode: &[u8]) -> Result<PathBuf> {
        let _ = color_eyre::install();

        let context = revmc::llvm::inkwell::context::Context::create();
        let backend = EvmLlvmBackend::new_for_target(
            &context,
            self.cfg.aot,
            self.cfg.opt_level,
            &revmc::Target::Native,
        )?;

        let mut compiler = EvmCompiler::new(backend);

        //let temp_dir = TempDir::new("jit_temp")?;
        //let temp_path = temp_dir.path();
        //fs::create_dir_all(&temp_path).await.unwrap();

        let temp_path = std::path::Path::new(JIT_OUT_PATH);
        std::fs::create_dir_all(&temp_path).unwrap();

        compiler.set_dump_to(Some(temp_path.to_path_buf()));
        compiler.gas_metering(self.cfg.no_gas);

        unsafe { compiler.stack_bound_checks(self.cfg.no_len_checks) };

        compiler.frame_pointers(true);
        compiler.debug_assertions(self.cfg.debug_assertions);
        compiler.set_module_name(self.unit.name);
        compiler.validate_eof(true);

        let spec_id = if self.cfg.eof {
            SpecId::OSAKA
        } else {
            self.cfg.spec_id.into()
        };

        if !self.unit.stack_input.is_empty() {
            compiler.inspect_stack_length(true);
        }

        let _f_id = compiler.translate(self.unit.name, bytecode, spec_id)?;

        let out_dir = std::env::temp_dir()
            .join(JIT_OUT_PATH)
            .join(&self.unit.name);
        std::fs::create_dir_all(&out_dir)?;

        // Compile.
        let obj = out_dir.join("a.o");
        compiler.write_object_to_file(&obj)?;
        ensure!(obj.exists(), "Failed to write object file");

        // Link.
        let so_path = out_dir.join("a.so");
        if !self.cfg.no_link {
            let linker = revmc::Linker::new();
            linker.link(&so_path, [obj.to_str().unwrap()])?;
            ensure!(so_path.exists(), "Failed to link object file");
        }

        Ok(so_path)
    }
}
