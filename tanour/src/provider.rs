use crate::types::{Bytes, Address, Hash32};
use crate::error::Error;

pub struct AccountInfo {
    pub nonce: u64,
    pub balance: u64,
    pub code: Bytes,
}

pub trait Provider {
    fn exists(&self, address: &Address) -> bool;
    fn account_info(&self, address: &Address) -> Result<AccountInfo, Error>;
    fn update_account(&mut self, address: &Address, bal: u64, nonce: u64) -> Result<(), Error>;
    fn create_contract(&mut self, address: &Address, code: &Bytes) -> Result<(), Error>;
    fn get_storage(&self, address: &Address, key: &Hash32) -> Result<Bytes, Error>;
    fn set_storage(&mut self, address: &Address, key: &Hash32, value: &Bytes) -> Result<(), Error>;
    fn block_number(&self) -> u64;
    fn gas_limit(&self) -> u64;
}
