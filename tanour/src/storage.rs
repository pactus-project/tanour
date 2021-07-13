use crate::{provider::StorageProvider, types::Bytes};
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
    chunks: HashMap<i32, ChunkData>,
    provider: P,
}

impl<P> Storage<P>
where
    P: StorageProvider,
{
    pub fn new(provider: P) -> Self {
        Storage {
            chunks: HashMap::new(),
            provider,
        }
    }
}
