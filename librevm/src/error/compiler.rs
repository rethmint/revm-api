use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Backend init error, err: {err}")]
    BackendInitError { err: String },

    #[error("File I/O error, err: {err}")]
    FileIOError { err: String },

    #[error("Bytecode translation error, err: {err}")]
    BytecodeTranslationError { err: String },

    #[error("Link error, err: {err}")]
    LinkError { err: String },
}
