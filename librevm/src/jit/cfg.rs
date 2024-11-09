use revm::primitives::SpecId;
use revmc::OptimizationLevel;

pub struct RuntimeJitCfg {
    pub target: &'static str,
    pub target_cpu: Option<String>,
    pub target_features: Option<String>,
    pub aot: bool,
    pub opt_level: OptimizationLevel,
    pub out_dir: &'static str,
    pub no_gas: bool,
    pub no_len_checks: bool,
    pub debug_assertions: bool,
    pub no_validate: bool,
    pub calldata: Option<String>,
    pub gas_limit: u64,
    pub eof: bool,
    pub spec_id: SpecId,
    pub bench_name: &'static str,
    pub no_link: bool,
}

impl Default for RuntimeJitCfg {
    fn default() -> Self {
        RuntimeJitCfg {
            target: "native",
            target_cpu: None,
            target_features: None,
            aot: true,
            opt_level: OptimizationLevel::Aggressive,
            out_dir: "out",
            no_gas: false,
            no_len_checks: false,
            debug_assertions: true,
            no_validate: false,
            calldata: None,
            gas_limit: 1_000_000_000,
            eof: true,
            spec_id: SpecId::OSAKA,
            bench_name: "Fibonacci",
            no_link: false,
        }
    }
}
