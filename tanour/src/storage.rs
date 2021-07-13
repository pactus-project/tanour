use crate::{error::Result, provider::StorageProvider, types::{Address, Bytes}};
use std::collections::HashMap;

#[derive(Debug)]
struct ChunkData {
    offset: i32,
    length: i32,
    data: Bytes,
    updated: bool,
}

#[derive(Debug)]
pub struct Storage<P> {
    address: Address,
    chunks: HashMap<i32, ChunkData>,
    provider: P,
}

impl<P> Storage<P>
where
    P: StorageProvider,
{
    pub fn new(address: Address, provider: P) -> Self {
        Storage {
            address,
            chunks: HashMap::new(),
            provider,
        }
    }

    fn read_storage(&self, offset: i64) -> Result<Bytes> {
        Ok(Vec::new())
    }
    fn write_storage(&mut self, offset: i64, value: &Bytes) -> Result<()> {
        Ok(())
    }
}
