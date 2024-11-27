use color_eyre::Result;
use revm::primitives::SpecId;
use revmc::OptimizationLevel;
use revmc::{eyre::ensure, EvmCompiler, EvmLlvmBackend};
use std::env;
use std::path::PathBuf;

fn aot_out_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".rethmint").join("output")
}

pub struct RuntimeAot {
    pub cfg: AotCfg,
}

impl RuntimeAot {
    pub fn new(cfg: AotCfg) -> Self {
        Self { cfg }
    }

    pub fn compile(&self, name: &'static str, bytecode: &[u8]) -> Result<PathBuf> {
        let _ = color_eyre::install();

        let context = revmc::llvm::inkwell::context::Context::create();
        let backend = EvmLlvmBackend::new_for_target(
            &context,
            self.cfg.aot,
            self.cfg.opt_level,
            &revmc::Target::Native,
        )?;

        let mut compiler = EvmCompiler::new(backend);

        let out_dir = aot_out_path();
        std::fs::create_dir_all(&out_dir).unwrap();

        compiler.set_dump_to(Some(out_dir.clone()));
        compiler.gas_metering(self.cfg.no_gas);

        unsafe {
            compiler.stack_bound_checks(self.cfg.no_len_checks);
        }

        compiler.frame_pointers(true);
        compiler.debug_assertions(self.cfg.debug_assertions);
        compiler.set_module_name(name);
        compiler.validate_eof(true);

        let spec_id = self.cfg.spec_id;

        compiler.inspect_stack_length(true);
        let _f_id = compiler.translate(name, bytecode, spec_id)?;

        let module_out_dir = std::env::temp_dir().join(out_dir).join(name);
        std::fs::create_dir_all(&module_out_dir)?;

        // Compile.
        let obj = module_out_dir.join("a.o");
        compiler.write_object_to_file(&obj)?;
        ensure!(obj.exists(), "Failed to write object file");

        // Link.
        let so_path = module_out_dir.join("a.so");
        let linker = revmc::Linker::new();
        linker.link(&so_path, [obj.to_str().unwrap()])?;
        ensure!(so_path.exists(), "Failed to link object file");

        Ok(so_path)
    }
}

pub struct AotCfg {
    pub aot: bool,
    pub opt_level: OptimizationLevel,
    pub no_gas: bool,
    pub no_len_checks: bool,
    pub debug_assertions: bool,
    pub spec_id: SpecId,
}

impl Default for AotCfg {
    fn default() -> Self {
        AotCfg {
            aot: true,
            opt_level: OptimizationLevel::Aggressive,
            no_gas: true,
            no_len_checks: true,
            debug_assertions: true,
            spec_id: SpecId::PRAGUE,
        }
    }
}
