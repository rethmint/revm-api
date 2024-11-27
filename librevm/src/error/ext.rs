use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtError {
    #[error("Operating on db, err: {err}")]
    DBError { err: String },

    #[error("IVec to pathbuf error")]
    IVecCastError,

    #[error("Lib loading error: {err}")]
    LibLoadingError { err: String },

    #[error("Get symbol error: {err}")]
    GetSymbolError { err: String },
}
