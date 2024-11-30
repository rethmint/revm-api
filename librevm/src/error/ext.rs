use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtError {
    #[error("Lib loading error: {err}")]
    LibLoadingError { err: String },

    #[error("Get symbol error: {err}")]
    GetSymbolError { err: String },
}
