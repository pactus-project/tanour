use crate::types::{Address, Bytes};

/// Action parameters
#[derive(Clone, Debug)]
pub struct Action {
    /// Address of currently executed code.
    pub code_address: Address,
    /// Hash of currently executed code.
    pub caller: Address,
    /// Gas limit.
    pub gas_limit: u64,
    /// Memory limit
    pub memory_limit: u64,
    /// Transaction value.
    pub value: u64,
    /// Code being executed.
    pub code: Bytes,
    /// Arguments
    pub args: Bytes,
}
