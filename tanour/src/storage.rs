use crate::types::Bytes;
use std::collections::HashMap;

#[derive(Debug)]
struct ChunkData {
    offset: i32,
    length: i32,
    data: Bytes,
    updated: bool,
}

#[derive(Debug)]
pub struct Storage {
    chunks: HashMap<i32, ChunkData>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            chunks: HashMap::new(),
        }
    }
}
