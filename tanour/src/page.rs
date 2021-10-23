use crate::types::Bytes;

#[derive(Debug)]
pub struct Page {
    pub offset: usize,
    pub length: usize,
    pub data: Bytes,
    pub updated: bool,
}
impl Page {
    pub fn new(offset: usize, length: usize, data: Bytes) -> Self {
        Page {
            offset,
            length,
            data,
            updated: false,
        }
    }
}
