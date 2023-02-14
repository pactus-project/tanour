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
    env.provider.lock().unwrap().write_storage(offset, &data)?;
    Ok(0)
}

pub(super) fn native_read_storage(
    func_env: FunctionEnvMut<Env>,
    offset: u32,
    ptr: u32,
    len: u32,
) -> Result<u32> {
    let env = func_env.data();

    let data = env.provider.lock().unwrap().read_storage(offset, len)?;
    memory::write_ptr(
        env.memory.as_ref().unwrap(),
        &func_env.as_store_ref(),
        ptr,
        &data,
    )?;
    Ok(0)
}

pub(super) fn native_get_param(
    func_env: FunctionEnvMut<Env>,
    _offset: u32,
    _ptr: u32,
    _len: u32,
) -> Result<u32> {
    let _address = [0; 21]; // TODO:
    let _env = func_env.data();

    // env.provider.lock().unwrap().exist(&address)?;

    Ok(0)
}
