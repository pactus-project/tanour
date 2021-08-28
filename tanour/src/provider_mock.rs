#[cfg(test)]
use crate::error::Error;
use crate::provider_api::*;
use crate::types::{Address, Bytes};

#[derive(Debug, Clone)]
pub struct ProviderMock {
    storage: Bytes,
}

impl ProviderMock {
    pub fn new(storage_size: usize) -> Self {
        let mut storage= Vec::with_capacity(storage_size);
        for _ in 0..storage_size {
            storage.push(0);
        }
        ProviderMock {
            storage,
        }
    }
}

impl ProviderAPI for ProviderMock {
    fn read_storage(&self, _address: &Address, offset: usize, length: usize) -> Result<Bytes> {
        if length + offset > self.storage.len() {
            return Err(Error::StorageError{msg: "Invalid offset".to_owned()});
        }

        let data = &self.storage[offset..offset+length];
        Ok(data.to_vec())
    }

    fn write_storage(&mut self, _address: &Address, _offset: usize, _value: &Bytes) -> Result<()> {
        Err(Error::CompileError {
            msg: "unimplemented".to_owned(),
        })
    }

    fn query(&self, _query: &Bytes) -> Result<Bytes> {
        Err(Error::CompileError {
            msg: "unimplemented".to_owned(),
        })
    }
}
