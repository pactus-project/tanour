use crate::{error::Result, memory::Pointer};

pub trait Executor {
    /// Calls a function with the given arguments.
    fn call_fn_0(&self, name: &str, arg: u64) -> Result<()>;

    /// Calls a function with the given arguments.
    fn call_fn_1(&self, name: &str, arg: u64) -> Result<u64>;

    /// Calls a function with the given arguments.
    fn call_fn_2(&self, name: &str, arg: u32) -> Result<u64>;

    // Writes data into wasm memory. Memory should be allocated before
    fn write_ptr(&self, ptr: &Pointer, data: &[u8]) -> Result<()>;

    // Read from wasm memory.
    fn read_ptr(&self, ptr: &Pointer) -> Result<Vec<u8>>;

    // Get the remaining points (metering)
    fn remaining_points(&self) -> Result<u64>;

    // Get the consumed points (metering)
    fn consumed_points(&self) -> Result<u64>;

    // Check if all points are consumed (metering)
    fn exhausted(&self) -> Result<bool>;
}
