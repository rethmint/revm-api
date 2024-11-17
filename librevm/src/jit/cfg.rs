use revm::primitives::SpecId;
use revmc::OptimizationLevel;

pub struct JitCfg {
    pub target: &'static str,
    pub target_cpu: Option<String>,
    pub target_features: Option<String>,
    pub aot: bool,
    pub opt_level: OptimizationLevel,
    pub no_gas: bool,
    pub no_len_checks: bool,
    pub debug_assertions: bool,
    pub eof: bool,
    pub spec_id: SpecId,
    pub no_link: bool,
}

impl Default for JitCfg {
    fn default() -> Self {
        JitCfg {
            target: "native",
            target_cpu: None,
            target_features: None,
            aot: true,
            opt_level: OptimizationLevel::Aggressive,
            no_gas: false,
            no_len_checks: false,
            debug_assertions: true,
            eof: true,
            spec_id: SpecId::PRAGUE,
            no_link: false,
        }
    }
}
