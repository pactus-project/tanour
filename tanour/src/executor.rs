use crate::{error::Result, types::Bytes};

pub trait Executor {
    /// Calls a function with the given arguments.
    fn call_fn_1(&self, name: &str, arg: u32) -> Result<()>;

    /// Calls a function with the given arguments.
    fn call_fn_2(&self, name: &str, arg: u32) -> Result<u32>;

    /// Calls a function with the given arguments.
    fn call_fn_3(&self, name: &str, arg1: u32, arg2: u32) -> Result<u64>;

    fn write_ptr(&self, ptr: u32, data: &[u8]) -> Result<()>;
    fn read_ptr(&self, ptr: u32, len: usize) -> Result<Bytes>;
}
