use crate::types::{Address, Hash32};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error instantiating module: {}", msg)]
    InstantiationError { msg: String },

    #[error("Error compiling module: {}", msg)]
    CompileError { msg: String },

    // #[error("Not supported")]
    // NotSupported,

    // #[error("Memory access violation")]
    // MemoryAccessViolation,

    // #[error("WASM Internal error: {}", msg)]
    // Wasm { msg: String },

    // #[error("Storage read error")]
    // StorageReadError,

    // #[error("Storage update error")]
    // StorageUpdateError,

    // #[error("Attempt to suicide resulted in an error")]
    // Suicide,

    // #[error("Return result")]
    // Return,

    // #[error("Suicide result")]
    // SuicideAbort,

    // #[error("Invalid gas state")]
    // InvalidGasState,

    // #[error("Balance query resulted in an error")]
    // BalanceQueryError,

    // #[error("Memory allocation failed (OOM)")]
    // AllocationFailed,

    // #[error("Invocation resulted in gas limit violated")]
    // GasLimit,

    // #[error("Unknown runtime function invoked")]
    // Unknown,

    // #[error("String encoding is bad utf-8 sequence")]
    // BadUtf8,

    // #[error("Error occurred while logging an event")]
    // Log,

    // #[error("Error: {}", msg)]
    // Other { msg: String },

    // #[error("Unreachable instruction encountered")]
    // Unreachable,

    // #[error("Invalid virtual call")]
    // InvalidVirtualCall,

    // #[error("Division by zero")]
    // DivisionByZero,

    // #[error("Invalid conversion to integer")]
    // InvalidConversionToInt,

    // #[error("Stack overflow")]
    // StackOverflow,

    // #[error("Panic: {}", msg)]
    // Panic { msg: String },

    // #[error("Account not found")]
    // AccountNotFound,

    // #[error("Storage key not found")]
    // KeyNotFound,
}
