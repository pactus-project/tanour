use crate::types::{Address, Hash32};
use snafu::Snafu;
use wasmi::TrapKind;

#[derive(Debug, Clone, Snafu)]
pub enum Error {
    #[snafu(display("Not supported"))]
    NotSupported,

    #[snafu(display("Memory access violation"))]
    MemoryAccessViolation,

    #[snafu(display("WASM Internal error: {}", msg))]
    Wasm { msg: String },

    #[snafu(display("Storage read error"))]
    StorageReadError,

    #[snafu(display("Storage update error"))]
    StorageUpdateError,

    #[snafu(display("Attempt to suicide resulted in an error"))]
    Suicide,

    #[snafu(display("Return result"))]
    Return,

    #[snafu(display("Suicide result"))]
    SuicideAbort,

    #[snafu(display("Invalid gas state"))]
    InvalidGasState,

    #[snafu(display("Balance query resulted in an error"))]
    BalanceQueryError,

    #[snafu(display("Memory allocation failed (OOM)"))]
    AllocationFailed,

    #[snafu(display("Invocation resulted in gas limit violated"))]
    GasLimit,

    #[snafu(display("Unknown runtime function invoked"))]
    Unknown,

    #[snafu(display("String encoding is bad utf-8 sequence"))]
    BadUtf8,

    #[snafu(display("Error occurred while logging an event"))]
    Log,

    #[snafu(display("Error: {}", msg))]
    Other { msg: String },

    #[snafu(display("Unreachable instruction encountered"))]
    Unreachable,

    #[snafu(display("Invalid virtual call"))]
    InvalidVirtualCall,

    #[snafu(display("Division by zero"))]
    DivisionByZero,

    #[snafu(display("Invalid conversion to integer"))]
    InvalidConversionToInt,

    #[snafu(display("Stack overflow"))]
    StackOverflow,

    #[snafu(display("Panic: {}", msg))]
    Panic { msg: String },

    #[snafu(display("Account not found"))]
    AccountNotFound,

    #[snafu(display("Storage key not found"))]
    KeyNotFound,
}

impl From<wasmi::Trap> for Error {
    fn from(trap: wasmi::Trap) -> Self {
        match *trap.kind() {
            TrapKind::Unreachable => Error::Unreachable,
            TrapKind::MemoryAccessOutOfBounds => Error::MemoryAccessViolation,
            TrapKind::TableAccessOutOfBounds | TrapKind::ElemUninitialized => {
                Error::InvalidVirtualCall
            }
            TrapKind::DivisionByZero => Error::DivisionByZero,
            TrapKind::InvalidConversionToInt => Error::InvalidConversionToInt,
            TrapKind::UnexpectedSignature => Error::InvalidVirtualCall,
            TrapKind::StackOverflow => Error::StackOverflow,
            TrapKind::Host(_) => Error::Other {
                msg: "Host error".to_string(),
            },
        }
    }
}

impl From<wasmi::Error> for Error {
    fn from(err: wasmi::Error) -> Self {
        match err {
            wasmi::Error::Validation(msg) => Error::Wasm {
                msg: format!("Wasm validation error: {}", msg),
            },
            wasmi::Error::Instantiation(msg) => Error::Wasm {
                msg: format!("Wasm Instantiation error: {}", msg),
            },
            wasmi::Error::Function(msg) => Error::Wasm {
                msg: format!("Wasm Function error: {}", msg),
            },
            wasmi::Error::Table(msg) => Error::Wasm {
                msg: format!("Wasm Table error: {}", msg),
            },
            wasmi::Error::Memory(msg) => Error::Wasm {
                msg: format!("Wasm Memory error: {}", msg),
            },
            wasmi::Error::Global(msg) => Error::Wasm {
                msg: format!("Wasm Global error: {}", msg),
            },
            wasmi::Error::Value(msg) => Error::Wasm {
                msg: format!("Wasm Value error: {}", msg),
            },
            wasmi::Error::Trap(k) => Error::Wasm {
                msg: format!("Wasm Trap error: {}", k),
            },
            wasmi::Error::Host(_) => Error::Wasm {
                msg: format!("Wasm Host error."),
            },
        }
    }
}

impl wasmi::HostError for Error {}
