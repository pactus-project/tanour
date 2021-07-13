use crate::{
    error::{Error, Result},
    types::{Address, Bytes},
};

pub trait StorageProvider {
    fn read_storage(&self, address: &Address, offset: i64) -> Result<Bytes>;
    fn write_storage(&mut self, address: &Address, offset: i64, value: &Bytes) -> Result<()>;
}

pub trait BlockchainProvider {
    fn query(&self, query: &Bytes) -> Result<Bytes>;
}

pub trait Provider: BlockchainProvider + StorageProvider + Copy {}
impl<P> Provider for P where P: BlockchainProvider + StorageProvider + Copy {}

#[cfg(test)]
#[path = "./provider_test.rs"]
pub mod provider_test;
