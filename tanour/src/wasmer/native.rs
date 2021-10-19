use super::env::Env;
use crate::{
    error::{Error, Result},
    executor::Executor,
};

pub fn native_write_storage(env: &Env, offset: u32, ptr: u32, length: u32) -> Result<u32>
{
    Ok(0)
}
