use super::{env::Env, memory};
use crate::error::Result;

pub fn native_write_storage(env: &Env, offset: u32, ptr: u32, len: u32) -> Result<u32> {
    let mem = env.memory()?;
    let data = memory::read_ptr(&mem, ptr, len)?;
    env.state
        .lock()
        .unwrap()
        .write_storage(offset as usize, &data)?;
    Ok(0)
}

pub fn native_read_storage(env: &Env, offset: u32, ptr: u32, len: u32) -> Result<u32> {
    let mem = env.memory()?;
    let data = env
        .state
        .lock()
        .unwrap()
        .read_storage(offset as usize, len as usize)?;
    memory::write_ptr(&mem, ptr, &data)?;
    Ok(0)
}

pub fn native_get_param(_env: &Env, _offset: u32, _ptr: u32, _len: u32) -> Result<u32> {
    todo!()
}
