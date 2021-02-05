use super::types::{Address, Bytes, Hash32};
use parity_wasm::peek_size;

#[derive(Debug, Clone)]
pub enum Action {
    /// Create creates new contract.
    /// Code + salt
    Create(Bytes, Hash32),
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(Address),
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: Address,
    pub value: u64,
    pub gas: u64,
    pub gas_price: u64,
    pub action: Action,
    pub args: Bytes,
}

impl Transaction {
    pub fn make_create_embedded_code(
        sender: Address,
        value: u64,
        gas: u64,
        gas_price: u64,
        code_params: Bytes,
        salt: Hash32,
    ) -> Self {
        let module_size = peek_size(&*code_params);
        let code = code_params[..module_size].to_vec();
        let args = code_params[module_size..].to_vec();

        Transaction {
            action: Action::Create(code, salt),
            sender,
            value,
            gas,
            gas_price,
            args,
        }
    }

    pub fn make_create(
        sender: Address,
        value: u64,
        gas: u64,
        gas_price: u64,
        code: Bytes,
        args: Bytes,
        salt: Hash32,
    ) -> Self {
        Transaction {
            action: Action::Create(code, salt),
            sender,
            value,
            gas,
            gas_price,
            args,
        }
    }

    pub fn make_call(
        sender: Address,
        contract: Address,
        value: u64,
        gas: u64,
        gas_price: u64,
        args: Bytes,
    ) -> Self {
        Transaction {
            action: Action::Call(contract),
            sender,
            value,
            gas,
            gas_price,
            args,
        }
    }
}
