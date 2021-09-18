use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Instantiating error: {}", msg)]
    InstantiationError { msg: String },

    #[error("Compile error: {}", msg)]
    CompileError { msg: String },

    #[error("Runtime error: {}", msg)]
    RuntimeError { msg: String },

    #[error("Storage error: {}", msg)]
    StorageError { msg: String },

    #[error("Memory error: {}", msg)]
    MemoryError { msg: String },

    #[error("Invalid memory region: {}", msg)]
    InvalidRegion { msg: String },
}
pub type Result<T> = core::result::Result<T, Error>;
