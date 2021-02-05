use crate::types::{Address, Bytes, Hash32};

/// The type of the instruction.
#[derive(Debug, PartialEq, Clone)]
pub enum ActionType {
    Create,
    Call,
}

/// Action (call/create) input params. Everything else should be specified in Externalities.
#[derive(Clone, Debug)]
pub struct ActionParams {
    /// Address of currently executed code.
    pub code_address: Address,
    /// Hash of currently executed code.
    pub code_hash: Option<Hash32>,
    /// Receive address. Usually equal to code_address,
    /// except when called using CALLCODE.
    pub address: Address,
    /// Sender of current part of the transaction.
    pub sender: Address,
    /// Transaction initiator.
    pub origin: Address,
    /// Gas paid up front for transaction execution
    pub gas: u64,
    /// Gas price.
    pub gas_price: u64,
    /// Transaction value.
    pub value: u64,
    /// Code being executed.
    pub code: Bytes,
    /// Arguments
    pub args: Bytes,
    /// Type of action (e.g. CALL, CREATE, etc.)
    pub action_type: ActionType,
}
