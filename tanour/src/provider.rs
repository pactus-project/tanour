use crate::error::Error;
use crate::types::Bytes;

pub trait Provider {
    fn read_storage(&self, offset: i64) -> Result<Bytes, Error>;
    fn write_storage(&mut self, offset: i64, value: &Bytes) -> Result<(), Error>;
    fn query(&self, lParam: i32, wParam: &Bytes) -> Result<(), Error>;
}
