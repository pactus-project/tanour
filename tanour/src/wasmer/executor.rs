use super::env::*;
use super::native::*;
use super::{compile, memory};
use crate::error::{Error, Result};
use crate::executor;
use crate::state::StateTrait;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::Mutex;
use wasmer::{Exports, Function, ImportObject, Val};

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct WasmerExecutor {
    /// We put this instance in a box to maintain a constant memory address for the entire
    /// lifetime of the instance in the cache. This is needed e.g. when linking the wasmer
    /// instance to a context.
    ///
    /// This instance should only be accessed via the Environment, which provides safe access.
    _instance: Box<wasmer::Instance>,
    env: Env,
}

impl WasmerExecutor {
    pub fn new(code: &[u8], memory_limit: u64, state: Arc<Mutex<dyn StateTrait>>) -> Result<Self> {
        let module = compile::compile(code, memory_limit)?;
        let store = module.store();
        let env = Env::new(state);
        let mut import_obj = ImportObject::new();
        let mut env_imports = Exports::new();

        env_imports.insert(
            "write_storage",
            Function::new_native_with_env(store, env.clone(), native_write_storage),
        );

        env_imports.insert(
            "read_storage",
            Function::new_native_with_env(store, env.clone(), native_read_storage),
        );

        import_obj.register("zarb", env_imports);

        let _instance = Box::new(wasmer::Instance::new(&module, &import_obj).map_err(
            |original| Error::InstantiationError {
                msg: format!("{:?}", original),
            },
        )?);

        let instance_ptr = NonNull::from(_instance.as_ref());
        env.set_instance(Some(instance_ptr));

        Ok(WasmerExecutor { env, _instance })
    }

    fn call_function(&self, name: &str, vals: &[Val]) -> Result<Box<[Val]>> {
        self.env.call_function(name, vals)
    }
}

impl executor::Executor for WasmerExecutor {
    fn call_fn_1(&self, name: &str, arg: u32) -> Result<()> {
        let val = wasmer::Val::I32(arg as i32);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => Err(Error::RuntimeError {
                msg: format!("Invalid return value for {}: {:?}", name, val),
            }),
            None => Ok(()),
        }
    }

    fn call_fn_2(&self, name: &str, arg: u32) -> Result<u32> {
        let val = wasmer::Val::I32(arg as i32);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => match val {
                Val::I32(i32) => Ok(*i32 as u32),
                _ => Err(Error::RuntimeError {
                    msg: format!("Invalid return value for {}", name),
                }),
            },
            None => Err(Error::RuntimeError {
                msg: format!("No return value for {}", name),
            }),
        }
    }

    fn call_fn_3(&self, name: &str, arg1: u32, arg2: u32) -> Result<u64> {
        let val1 = wasmer::Val::I32(arg1 as i32);
        let val2 = wasmer::Val::I32(arg2 as i32);
        let result = self.call_function(name, &[val1, val2])?;

        match result.first() {
            Some(val) => match val {
                Val::I64(i64) => Ok(*i64 as u64),
                _ => Err(Error::RuntimeError {
                    msg: format!("Invalid return value for {}", name),
                }),
            },
            None => Err(Error::RuntimeError {
                msg: format!("No return value for {}", name),
            }),
        }
    }

    fn write_ptr(&self, ptr: u32, data: &[u8]) -> Result<()> {
        memory::write_ptr(&self.env.memory()?, ptr, data)
    }

    fn read_ptr(&self, ptr: u32, len: usize) -> Result<Vec<u8>> {
        memory::read_ptr(&self.env.memory()?, ptr, len)
    }
}

#[cfg(test)]
#[path = "./executor_test.rs"]
mod tests;
