use crate::error::Error;
use crate::provider::Provider;
use crate::types::{Address, Bytes, Hash32};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, PartialEq)]
struct Account {
    nonce: u64,
    balance: u64,
    code: Bytes,
    storage: HashMap<Hash32, (Bytes, bool)>,
}

impl Account {
    pub fn new(nonce: u64, balance: u64, code: Bytes) -> Account {
        Account {
            nonce,
            balance,
            code,
            storage: HashMap::new(),
        }
    }
}

pub struct State<'a> {
    provider: &'a mut dyn Provider,
    accounts: HashMap<Address, (Account, bool)>,
}

impl<'a> State<'a> {
    pub fn new(provider: &'a mut dyn Provider) -> Self {
        State {
            provider: provider,
            accounts: HashMap::new(),
        }
    }


}
