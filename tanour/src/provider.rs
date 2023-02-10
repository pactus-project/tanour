use crate::blockchain_api::BlockchainAPI;
use crate::error::Result;
use crate::storage_file::StorageFile;
use crate::Address;

pub struct Provider {
    blockchain_api: Box<dyn BlockchainAPI>,
    storage_file: StorageFile,
}

impl Provider {
    pub fn new(blockchain_api: Box<dyn BlockchainAPI>, storage_file: StorageFile) -> Self {
        Provider {
            blockchain_api,
            storage_file,
        }
    }
    pub fn read_storage(&mut self, offset: u32, length: u32) -> Result<Vec<u8>> {
        self.storage_file.read_storage(offset, length)
    }

    pub fn write_storage(&mut self, offset: u32, data: &[u8]) -> Result<()> {
        self.storage_file.write_storage(offset, data)
    }

    pub fn exist(&self, address: &Address) -> Result<bool> {
        self.blockchain_api.exist(address)
    }
}
