use super::env::*;
use super::native::*;
use super::{compile, memory};
use crate::error::{Error, Result};
use crate::executor;
use crate::memory::Pointer;
use crate::state::StateTrait;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::Mutex;
use wasmer::{Exports, Function, ImportObject, Val};
use wasmer_middlewares::metering::MeteringPoints;

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

    /// We wrap state inside `Env` which we'll pass to the imported functions.
    env: Env,

    // The limit for metering middleware
    metering_limit: u64,
}

impl WasmerExecutor {
    pub fn new(
        code: &[u8],
        memory_limit: u64,
        metering_limit: u64,
        state: Arc<Mutex<dyn StateTrait>>,
    ) -> Result<Self> {
        let module = compile::compile(code, memory_limit, metering_limit)?;
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

        env_imports.insert(
            "get_param",
            Function::new_native_with_env(store, env.clone(), native_get_param),
        );

        import_obj.register("pactus", env_imports);

        let _instance = Box::new(wasmer::Instance::new(&module, &import_obj).map_err(
            |original| Error::InstantiationError {
                msg: format!("{:?}", original),
            },
        )?);

        let instance_ptr = NonNull::from(_instance.as_ref());
        env.set_instance(Some(instance_ptr));

        Ok(WasmerExecutor {
            _instance,
            env,
            metering_limit,
        })
    }

    fn call_function(&self, name: &str, vals: &[Val]) -> Result<Box<[Val]>> {
        self.env.call_function(name, vals)
    }
}

impl executor::Executor for WasmerExecutor {
    fn call_fn_0(&self, name: &str, arg: u64) -> Result<()> {
        let val = wasmer::Val::I64(arg as i64);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => Err(Error::RuntimeError {
                msg: format!("Invalid return value for {}: {:?}", name, val),
            }),
            None => Ok(()),
        }
    }

    fn call_fn_1(&self, name: &str, arg: u64) -> Result<u64> {
        let val = wasmer::Val::I64(arg as i64);
        let result = self.call_function(name, &[val])?;

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

    fn call_fn_2(&self, name: &str, arg: u32) -> Result<u64> {
        let val = wasmer::Val::I32(arg as i32);
        let result = self.call_function(name, &[val])?;

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

    fn write_ptr(&self, ptr: &Pointer, data: &[u8]) -> Result<()> {
        memory::write_ptr(&self.env.memory()?, ptr.offset(), data)
    }

    fn read_ptr(&self, ptr: &Pointer) -> Result<Vec<u8>> {
        memory::read_ptr(&self.env.memory()?, ptr.offset(), ptr.length())
    }

    fn remaining_points(&self) -> Result<u64> {
        match self.env.remaining_points()? {
            MeteringPoints::Exhausted => Ok(0),
            MeteringPoints::Remaining(points) => Ok(points),
        }
    }

    fn consumed_points(&self) -> Result<u64> {
        Ok(self.metering_limit - self.remaining_points()?)
    }

    fn exhausted(&self) -> Result<bool> {
        match self.env.remaining_points()? {
            MeteringPoints::Exhausted => Ok(true),
            MeteringPoints::Remaining(_) => Ok(false),
        }
    }
}
