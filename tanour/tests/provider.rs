use std::collections::hash_map::Entry;
use std::collections::HashMap;
use tanour::error::Error;
use tanour::provider::{Provider, AccountInfo};
use tanour::types::{Address, Bytes, Hash32};

#[derive(Debug, Clone)]
pub struct Account {
    pub nonce: u64,
    pub balance: u64,
    pub code: Bytes,
    pub storage: HashMap<Hash32, Bytes>,
}

pub struct Blockchain {
    accounts: HashMap<Address, Account>,
    block_number: u64,
    gas_limit: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            accounts: HashMap::new(),
            block_number: 0,
            gas_limit: 1000000,
        }
    }

    pub fn account(&self, address: &Address) -> Result<Account, Error> {
        match self.accounts.get(address) {
            Some(acc) => Ok(acc.clone()),
            None => Err(Error::AccountNotFound),
        }
    }

    pub fn add_account(&mut self, address: Address) {
         self.accounts.insert(address,
            Account {
            balance:0,
            code: vec![],
            nonce:0,
            storage:HashMap::new(),
        }) ;
    }
}

impl Provider for Blockchain {
    fn exists(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    fn account_info(&self, address: &Address) -> Result<AccountInfo, Error> {
        let acc = self.account(address)?;
        Ok(AccountInfo {
            balance: acc.balance,
            nonce: acc.nonce,
            code: acc.code.clone(),
        })
    }

    fn create_contract(&mut self, address: &Address, code: &Vec<u8>) -> Result<(), Error> {
        let acc = Account {
            nonce: 0,
            code: code.clone(),
            balance: 0,
            storage: HashMap::new(),
        };
        self.accounts.insert(*address, acc);
        Ok(())
    }

    fn update_account(&mut self, address: &Address, bal: u64, nonce: u64) -> Result<(), Error> {
        let entry = self.accounts.entry(*address);
        match entry {
            Entry::Occupied(mut e) => {
                let mut acc = e.get_mut();
                acc.balance = bal;
                acc.nonce = nonce;
                Ok(())
            }
            Entry::Vacant(_) => Err(Error::AccountNotFound),
        }
    }

    fn get_storage(&self, address: &Address, key: &Hash32) -> Result<Bytes, Error> {
        let acc = self.account(address)?;
        match acc.storage.get(key) {
            Some(storage) => Ok(storage.clone()),
            None => Err(Error::KeyNotFound),
        }
    }

    fn set_storage(&mut self, address: &Address, key: &Hash32, value: &Bytes) -> Result<(), Error> {
        let mut acc = self.account(address).unwrap();
        let val = acc.storage.entry(*key).or_insert(value.clone());
        *val = value.clone();
        Ok(())
    }

    fn block_number(&self) -> u64 {
        self.block_number
    }

    fn gas_limit(&self) -> u64 {
        self.gas_limit
    }
}
