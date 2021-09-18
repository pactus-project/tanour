use crate::error::Result;

pub trait Executor {
    /// Calls a function with the given arguments.
    fn call_fn_1(&self, name: &str, arg: u32) -> Result<()>;

    /// Calls a function with the given arguments.
    fn call_fn_2(&self, name: &str, arg: u32) -> Result<u32>;

    /// Calls a function with the given arguments.
    fn call_fn_3(&self, name: &str, arg1: u32, arg2: u32) -> Result<u32>;

    fn write_ptr(&self, ptr: u32, data: &[u8]) -> Result<()>;
    //fn read_ptr(&mut self, ptr: u32, lenght: u32, data: &[u8]) -> Result<()>;
}
