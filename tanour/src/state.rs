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

    pub fn nonce(&mut self, address: Address) -> Result<u64, Error> {
        let acc = self.account(address)?;
        Ok(acc.nonce)
    }

    pub fn balance(&mut self, address: Address) -> Result<u64, Error> {
        let acc = self.account(address)?;
        Ok(acc.balance)
    }

    pub fn exists(&self, address: Address) -> bool {
        self.provider.exists(&address)
    }

    pub fn block_number(&self) -> u64 {
        self.provider.block_number()
    }

    pub fn storage_read(&mut self, address: Address, key: Hash32) -> Result<Bytes, Error> {
        self.fetch_account(address)?;

        let acc = self.accounts.get_mut(&address).unwrap();

        match acc.0.storage.entry(key) {
            Entry::Occupied(occupied) => {
                let val = occupied.get();

                Ok(val.0.clone())
            },
            Entry::Vacant(vacant) => {
                let val = self.provider.get_storage(&address, &key)?;

                vacant.insert((val.clone(), false));
                Ok(val)
            }
        }
    }

    pub fn set_storage(
        &mut self,
        address: Address,
        key: Hash32,
        value: Bytes,
    ) -> Result<(), Error> {
        let acc = self.account_mut(address)?;
        acc.0.storage.insert(key, (value, true));

        Ok(())
    }

    pub fn init_code(&mut self, address: Address, code: Bytes) {
        let mut acc = self.account_mut(address).unwrap();
        acc.0.code = code;
        acc.1 = true;
    }

    fn account_mut(&mut self, address: Address) -> Result<&mut (Account, bool), Error> {
        self.fetch_account(address)?;

        return Ok(self.accounts.get_mut(&address).unwrap());
    }

    fn account(&mut self, address: Address) -> Result<&Account, Error> {
        self.fetch_account(address)?;

        return Ok(&self.accounts.get(&address).unwrap().0);
    }

    pub fn update_state(&mut self) -> Result<(), Error> {
        for (addr, acc) in &self.accounts {
            if acc.1 {
                if !self.provider.exists(addr) {
                    self.provider.create_contract(addr, &acc.0.code)?;
                } else {
                    self.provider
                        .update_account(addr, acc.0.balance, acc.0.nonce)?;
                }
            }

            for (key, val) in &acc.0.storage {
                if val.1 {
                    self.provider.set_storage(addr, key, &val.0)?;
                }
            }
        }

        Ok(())
    }

    fn fetch_account(&mut self, address: Address) -> Result<(), Error> {
        match self.accounts.entry(address) {
            Entry::Occupied(_) => Ok(()),
            Entry::Vacant(entry) => {
                let info = self.provider.account_info(&address)?;
                let acc = Account::new(info.balance, info.nonce, info.code);

                entry.insert((acc, false));
                Ok(())
            }
        }
    }
}
