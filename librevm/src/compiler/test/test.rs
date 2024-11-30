use revmc::OptimizationLevel;
use std::panic;

use super::utils::{count_reference, initiate_compiler_work};
use crate::compiler::test::utils::{init_sled_db, init_worker};

#[test]
fn test_compiler_init_sled_db() {
    let res = panic::catch_unwind(|| init_sled_db());

    assert!(res.is_ok())
}

#[test]
fn test_compiler_init_compile_worker() {
    let threshold = 1_000;
    let (_, worker) = init_worker(threshold);
    assert_eq!(threshold, worker.threshold);

    let aot_cfg = &worker.aot_runtime().cfg;
    assert_eq!(aot_cfg.aot, true);
    assert_eq!(aot_cfg.opt_level, OptimizationLevel::Aggressive);
    assert_eq!(aot_cfg.no_gas, true);
    assert_eq!(aot_cfg.no_len_checks, true);
    assert_eq!(aot_cfg.debug_assertions, true);
}

#[test]
fn test_compiler_worker_work() {
    let res = panic::catch_unwind(|| initiate_compiler_work());

    assert!(res.is_ok())
}

#[test]
fn test_increment_reference_count() {
    let (db, code_hash) = initiate_compiler_work();

    let _prev = count_reference(db, code_hash);
}
