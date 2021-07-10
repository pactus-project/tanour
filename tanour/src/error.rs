use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error instantiating module: {}", msg)]
    InstantiationError { msg: String },

    #[error("Error compiling module: {}", msg)]
    CompileError { msg: String },

    #[error("Error executing {}: {}", func_name, msg)]
    RuntimeError { func_name: String, msg: String },
}
pub type Result<T> = core::result::Result<T, Error>;
