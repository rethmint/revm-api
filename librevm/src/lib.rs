mod compiler;
mod db;
mod error;
mod evmdb;
mod ext;
mod gstorage;
mod interface;
mod memory;
mod runtime;
mod tracer;
mod utils;

use std::env;
use std::path::PathBuf;

pub use interface::*;

#[inline]
pub(crate) fn aot_out_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".rethmint").join("output")
}
