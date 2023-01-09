use super::{executor::Env, memory};
use crate::error::Result;
use wasmer::{AsStoreRef, FunctionEnvMut};

pub(super) fn native_write_storage(
    func_env: FunctionEnvMut<Env>,
    offset: u32,
    ptr: u32,
    len: u32,
) -> Result<u32> {
    let env = func_env.data();
    let data = memory::read_ptr(
        env.memory.as_ref().unwrap(),
        &func_env.as_store_ref(),
        ptr,
        len,
    )?;
    env.state
        .lock()
        .unwrap()
        .write_storage(offset as usize, &data)?;
    Ok(0)
}

pub(super) fn native_read_storage(
    func_env: FunctionEnvMut<Env>,
    offset: u32,
    ptr: u32,
    len: u32,
) -> Result<u32> {
    let env = func_env.data();

    let data = env
        .state
        .lock()
        .unwrap()
        .read_storage(offset as usize, len as usize)?;
    memory::write_ptr(
        env.memory.as_ref().unwrap(),
        &func_env.as_store_ref(),
        ptr,
        &data,
    )?;
    Ok(0)
}

pub(super) fn native_get_param(
    _func_env: FunctionEnvMut<Env>,
    _offset: u32,
    _ptr: u32,
    _len: u32,
) -> Result<u32> {
    todo!()
}
