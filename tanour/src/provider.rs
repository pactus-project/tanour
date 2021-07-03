use crate::error::Error;
use crate::types::{Address, Bytes};

pub trait Provider {
    fn read_storage(&self, address: &Address, offset: i64) -> Result<Bytes, Error>;
    fn write_storage(&mut self, address: &Address, offset: i64, value: &Bytes)
        -> Result<(), Error>;
    fn query(&self, query: &Bytes) -> Result<Bytes, Error>;
}
