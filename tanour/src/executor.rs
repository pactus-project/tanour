use crate::{error::Result, region::Region};

pub trait Executor {
    /// Calls a function with the given arguments.
    fn call_function(&self, name: &str, args_region: &Region) -> Result<Option<Region>>;
}
