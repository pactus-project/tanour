#[cfg(test)]
use crate::error::Error;
use crate::provider::Provider;
use crate::types::{Address, Bytes};

pub struct ProviderMock {}

impl Provider for ProviderMock {
    fn read_storage(&self, _address: &Address, _offset: i64) -> Result<Bytes, Error> {
        Err(Error::CompileError { msg: "unimplemented".to_owned() })
    }

    fn write_storage(
        &mut self,
        _address: &Address,
        _offset: i64,
        _value: &Bytes,
    ) -> Result<(), Error> {
        Err(Error::CompileError { msg: "unimplemented".to_owned() })
    }

    fn query(&self, _query: &Bytes) -> Result<Bytes, Error> {
        Err(Error::CompileError { msg: "unimplemented".to_owned() })
    }
}
