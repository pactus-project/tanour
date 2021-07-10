use crate::types::{Address, Bytes};

#[derive(Debug)]
pub enum CallMethod {
    /// Deploy creates and deploy a new contract.
    Deploy,
    /// Execute call the execute method
    Execute,
}

/// Action parameters
#[derive(Debug)]
pub struct Action {
    /// Hash of currently executed code.
    pub caller: Address,
    /// Transaction value.
    pub value: u64,
    /// Gas limit.
    pub gas_limit: u64,
    /// Method to be called
    pub method: CallMethod,
    /// Arguments
    pub args: Bytes,
}

impl Action {
    pub fn new(
        caller: Address,
        value: u64,
        gas_limit: u64,
        method: CallMethod,
        args: Bytes,
    ) -> Self {
        Action {
            caller,
            value,
            gas_limit,
            method,
            args,
        }
    }
}
