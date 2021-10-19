#[cfg(test)]
use crate::error::Error;
use crate::provider_api::*;
use crate::types::{Address, Bytes};

#[derive(Debug, Clone)]
pub struct ProviderMock {
    pub storage: Bytes,
}

impl ProviderMock {
    pub fn new(storage_size: usize) -> Self {
        let mut storage = Vec::with_capacity(storage_size);
        for _ in 0..storage_size {
            storage.push(0);
        }
        ProviderMock { storage }
    }
}

impl ProviderAPI for ProviderMock {
    fn read_storage(&self, offset: usize, length: usize) -> Result<Bytes> {
        if offset + length > self.storage.len() {
            return Err(Error::StorageReadError {
                msg: "Invalid offset".to_owned(),
            });
        }

        let data = &self.storage[offset..offset + length];
        Ok(data.to_vec())
    }

    fn write_storage(&mut self, offset: usize, value: &Bytes) -> Result<()> {
        // if offset + value.len() > self.storage.len() {
        //     return Err(Error::StorageWriteError {
        //         msg: "Invalid offset".to_owned(),
        //     });
        // }

        // for (i, d) in value.iter().enumerate() {
        //     self.storage[offset + i] = *d;
        // }
        // Ok(())

        todo!()
    }

    fn query(&self, _query: &Bytes) -> Result<Bytes> {
        todo!()
    }
}
