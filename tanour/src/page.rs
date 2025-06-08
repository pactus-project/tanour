#[derive(Debug)]
pub struct Page {
    pub data: Vec<u8>,
}
impl Page {
    pub fn new(data: Vec<u8>) -> Self {
        Page { data }
    }
}
