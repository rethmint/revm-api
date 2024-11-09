mod cfg;

use std::{path::PathBuf, str::FromStr};

use alloy_primitives::{address, hex, U256};
use cfg::RuntimeJitCfg;
use color_eyre::Result;
use revm::primitives::{Env, SpecId, TransactTo};
use revmc::{eyre::ensure, EvmCompiler, EvmCompilerFn, EvmLlvmBackend};

pub struct RuntimeJit {
    pub unit: CompileUnit,
}

#[derive(Clone, Debug, Default)]
pub struct CompileUnit {
    pub name: &'static str,
    pub bytecode: Vec<u8>,
    pub calldata: Vec<u8>,
    pub stack_input: Vec<U256>,
}

impl RuntimeJit {
    pub fn compile(&self, cfg: RuntimeJitCfg) -> Result<EvmCompilerFn> {
        if std::env::var_os("RUST_BACKTRACE").is_none() {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
        let _ = color_eyre::install();

        // Build the compiler.
        let context = revmc::llvm::inkwell::context::Context::create();
        let target = revmc::Target::new(cfg.target, cfg.target_cpu, cfg.target_features);
        let backend = EvmLlvmBackend::new_for_target(&context, cfg.aot, cfg.opt_level, &target)?;
        let mut compiler = EvmCompiler::new(backend);
        let out_pathbuf = PathBuf::from_str(cfg.out_dir)?;
        compiler.set_dump_to(Some(out_pathbuf));
        compiler.gas_metering(cfg.no_gas);
        unsafe { compiler.stack_bound_checks(cfg.no_len_checks) };
        compiler.frame_pointers(true);
        compiler.debug_assertions(cfg.debug_assertions);
        compiler.validate_eof(cfg.no_validate);

        let CompileUnit {
            name,
            bytecode,
            calldata,
            stack_input,
        } = CompileUnit {
            name: "fibonacci",
            bytecode: hex!(
                "5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3"
            )
            .to_vec(),
            stack_input: vec![U256::from(69)],
            ..Default::default()
        };

        compiler.set_module_name(name);

        let calldata = if let Some(calldata) = cfg.calldata {
            revmc::primitives::hex::decode(calldata)?.into()
        } else {
            calldata.into()
        };
        let gas_limit = cfg.gas_limit;

        let mut env = Env::default();
        env.tx.caller = address!("0000000000000000000000000000000000000001");
        env.tx.transact_to = TransactTo::Call(address!("0000000000000000000000000000000000000002"));
        env.tx.data = calldata;
        env.tx.gas_limit = gas_limit;

        let bytecode =
            revm::interpreter::analysis::to_analysed(revm::primitives::Bytecode::new_raw(
                revm::primitives::Bytes::copy_from_slice(&bytecode),
            ));
        let contract = revm::interpreter::Contract::new_env(&env, bytecode, None);

        let bytecode = contract.bytecode.original_byte_slice();

        let spec_id = if cfg.eof {
            SpecId::OSAKA
        } else {
            cfg.spec_id.into()
        };
        if !stack_input.is_empty() {
            compiler.inspect_stack_length(true);
        }

        let f_id = compiler.translate(name, bytecode, spec_id)?;

        if cfg.aot {
            let out_dir = if let Some(out_dir) = compiler.out_dir() {
                out_dir.join(cfg.bench_name)
            } else {
                let dir = std::env::temp_dir().join("revmc-cli").join(cfg.bench_name);
                std::fs::create_dir_all(&dir)?;
                dir
            };

            // Compile.
            let obj = out_dir.join("a.o");
            compiler.write_object_to_file(&obj)?;
            ensure!(obj.exists(), "Failed to write object file");
            eprintln!("Compiled object file to {}", obj.display());

            // Link.
            if !cfg.no_link {
                let so = out_dir.join("a.so");
                let linker = revmc::Linker::new();
                linker.link(&so, [obj.to_str().unwrap()])?;
                ensure!(so.exists(), "Failed to link object file");
                eprintln!("Linked shared object file to {}", so.display());
            }
        }

        let f = unsafe { compiler.jit_function(f_id)? };
        Ok(f)
    }
}
