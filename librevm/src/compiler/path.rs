use std::path::PathBuf;

#[inline]
fn default_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".aotstore")
}

#[inline]
pub fn aot_store_path() -> PathBuf {
    default_path().join("output")
}

#[inline]
pub fn sleddb_path() -> PathBuf {
    default_path().join("db")
}
