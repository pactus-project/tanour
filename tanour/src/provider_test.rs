#[cfg(test)]
use crate::error::Error;
use crate::provider::Provider;
use crate::types::{Address, Bytes};

pub struct ProviderMock {}

impl Provider for ProviderMock {
    fn read_storage(&self, address: &Address, offset: i64) -> Result<Bytes, Error> {
        Err(Error::CompileError { msg: "".to_owned() })
    }

    fn write_storage(
        &mut self,
        address: &Address,
        offset: i64,
        value: &Bytes,
    ) -> Result<(), Error> {
        Err(Error::CompileError { msg: "".to_owned() })
    }

    fn query(&self, query: &Bytes) -> Result<Bytes, Error> {
        Err(Error::CompileError { msg: "".to_owned() })
    }
}
