use crate::{
    error::Result,
    types::{Address, Bytes},
};

pub trait ProviderAPI {
    fn read_storage(&self, address: &Address, offset: usize, length: usize) -> Result<Bytes>;
    fn write_storage(&mut self, address: &Address, offset: usize, value: &Bytes) -> Result<()>;
    fn query(&self, query: &Bytes) -> Result<Bytes>;
}

#[cfg(test)]
#[path = "./provider_mock.rs"]
pub mod provider_mock;
