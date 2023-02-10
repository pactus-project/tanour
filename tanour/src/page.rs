#[derive(Debug)]
pub struct Page {
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>,
    pub updated: bool,
}
impl Page {
    pub fn new(offset: u32, length: u32, data: Vec<u8>) -> Self {
        Page {
            offset,
            length,
            data,
            updated: false,
        }
    }
}
