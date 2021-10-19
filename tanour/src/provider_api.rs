use crate::{
    error::Result,
    types::{Address, Bytes},
};

pub trait ProviderAPI {
    fn read_storage(&self, offset: usize, length: usize) -> Result<Bytes>;
    fn write_storage(&mut self, offset: usize, value: &Bytes) -> Result<()>;
    fn query(&self, query: &Bytes) -> Result<Bytes>;
}

#[cfg(test)]
#[path = "./provider_mock.rs"]
pub mod provider_mock;
