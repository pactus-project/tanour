#[cfg(test)]
use crate::error::Error;
use crate::provider::*;
use crate::types::{Address, Bytes};

#[derive(Copy, Clone)]
pub struct ProviderMock {}

impl StorageProvider for ProviderMock {
    fn read_storage(&self, _address: &Address, _offset: i64) -> Result<Bytes, Error> {
        Err(Error::CompileError {
            msg: "unimplemented".to_owned(),
        })
    }

    fn write_storage(
        &mut self,
        _address: &Address,
        _offset: i64,
        _value: &Bytes,
    ) -> Result<(), Error> {
        Err(Error::CompileError {
            msg: "unimplemented".to_owned(),
        })
    }
}

impl BlockchainProvider for ProviderMock {
    fn query(&self, _query: &Bytes) -> Result<Bytes, Error> {
        Err(Error::CompileError {
            msg: "unimplemented".to_owned(),
        })
    }
}
