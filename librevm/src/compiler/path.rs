use std::path::PathBuf;

#[inline]
pub(crate) fn aot_out_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home_dir).join(".rethmint").join("output")
}
