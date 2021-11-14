use crate::error::Result;

pub trait ProviderAPI: Send + Sync + 'static {
    fn read_storage(&self, offset: usize, length: usize) -> Result<Vec<u8>>;
    fn write_storage(&mut self, offset: usize, data: &[u8]) -> Result<()>;
    fn query(&self, query: &[u8]) -> Result<Vec<u8>>;
}
