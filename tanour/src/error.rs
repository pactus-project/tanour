use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Instantiating error: {msg}")]
    InstantiationError { msg: String },

    #[error("Compile error: {msg}")]
    CompileError { msg: String },

    #[error("Runtime error: {msg}")]
    RuntimeError { msg: String },

    #[error("Memory error: {msg}")]
    MemoryError { msg: String },

    #[error("Serialization error: {msg}")]
    SerializationError { msg: String },

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("NetworkError error: {}", msg)]
    NetworkError { msg: String },
}
pub type Result<T> = std::result::Result<T, Error>;
