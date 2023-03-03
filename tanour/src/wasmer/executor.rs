use super::compile;
use super::memory;
use super::native::*;
use crate::error::{Error, Result};
use crate::executor;
use crate::memory::Pointer;
use crate::provider::Provider;
use std::sync::Arc;
use std::sync::Mutex;
use wasmer::AsStoreRef;
use wasmer::Memory;
use wasmer::Store;
use wasmer::{imports, AsStoreMut, Function, FunctionEnv, Value};
use wasmer_middlewares::metering::{get_remaining_points, MeteringPoints};

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

#[derive(Clone)]
pub(super) struct Env {
    pub provider: Arc<Mutex<dyn Provider>>,
    pub memory: Option<Memory>,
}

pub struct WasmerExecutor {
    instance: wasmer::Instance,
    store_lock: Arc<Mutex<Store>>,

    // The limit for metering middleware
    metering_limit: u64,
}

impl WasmerExecutor {
    /// creates the new instance of WASMER executor
    /// `code` should be the wat byte codes
    /// `memory_limit_page` is the maximum a linear memory is allowed to be (in Wasm pages, 64 KiB each).
    /// `metering_limit` is the maximum operator that can be  executed in total.
    pub fn new(
        code: &[u8],
        memory_limit_page: u32,
        metering_limit: u64,
        provider: Arc<Mutex<dyn Provider>>,
    ) -> Result<Self> {
        let (module, store) = compile::compile(code, memory_limit_page, metering_limit)?;
        let store_lock = Arc::new(Mutex::new(store));
        let mut store_guard = store_lock.lock().unwrap();

        let env = Env {
            provider,
            memory: None,
        };
        let fun_env = FunctionEnv::new(&mut store_guard.as_store_mut(), env);

        // Create an import object.
        let import_object = imports! {
            "pactus" => {
                "write_storage" => Function::new_typed_with_env(&mut store_guard.as_store_mut(), &fun_env, native_write_storage),
                "read_storage" => Function::new_typed_with_env(&mut store_guard.as_store_mut(), &fun_env, native_read_storage),
                "get_param" => Function::new_typed_with_env(&mut store_guard.as_store_mut(), &fun_env, native_get_param),
            }
        };

        let instance =
            wasmer::Instance::new(&mut store_guard.as_store_mut(), &module, &import_object)
                .map_err(|original| Error::InstantiationError {
                    msg: format!("{original}"),
                })?;

        fun_env.as_mut(&mut store_guard.as_store_mut()).memory = Some(
            instance
                .exports
                .get_memory("memory")
                .map_err(|original| Error::InstantiationError {
                    msg: format!("{original}"),
                })?
                .clone(),
        );

        Ok(WasmerExecutor {
            instance,
            store_lock: store_lock.clone(),
            metering_limit,
        })
    }

    fn call_function(&self, name: &str, vals: &[Value]) -> Result<Box<[Value]>> {
        let func = self
            .instance
            .exports
            .get_function(name)
            .map_err(|original| Error::RuntimeError {
                msg: format!("{original}"),
            })?;

        let mut store_guard = self
            .store_lock
            .lock()
            .map_err(|original| Error::RuntimeError {
                msg: format!("{original}"),
            })?;

        func.call(&mut store_guard.as_store_mut(), vals)
            .map_err(|original| Error::RuntimeError {
                msg: format!("{original}"),
            })
    }

    fn memory(&self) -> Result<&Memory> {
        self.instance
            .exports
            .get_memory("memory")
            .map_err(|original| Error::RuntimeError {
                msg: format!("{original}"),
            })
    }
}

impl executor::Executor for WasmerExecutor {
    fn call_fn_0(&self, name: &str, arg: u64) -> Result<()> {
        let val = Value::I64(arg as i64);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => Err(Error::RuntimeError {
                msg: format!("Invalid return value for {name}: {val:?}"),
            }),
            None => Ok(()),
        }
    }

    fn call_fn_1(&self, name: &str, arg: u64) -> Result<u64> {
        let val = Value::I64(arg as i64);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => match val {
                Value::I64(i64) => Ok(*i64 as u64),
                _ => Err(Error::RuntimeError {
                    msg: format!("Invalid return value for {name}"),
                }),
            },
            None => Err(Error::RuntimeError {
                msg: format!("No return value for {name}"),
            }),
        }
    }

    fn call_fn_2(&self, name: &str, arg: u32) -> Result<u64> {
        let val = wasmer::Value::I32(arg as i32);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => match val {
                Value::I64(i64) => Ok(*i64 as u64),
                _ => Err(Error::RuntimeError {
                    msg: format!("Invalid return value for {name}"),
                }),
            },
            None => Err(Error::RuntimeError {
                msg: format!("No return value for {name}"),
            }),
        }
    }

    fn write_ptr(&self, ptr: &Pointer, data: &[u8]) -> Result<()> {
        let store_guard =
            self.store_lock
                .as_ref()
                .lock()
                .map_err(|original| Error::RuntimeError {
                    msg: format!("{original}"),
                })?;

        let memory = self.memory()?;
        memory::write_ptr(memory, &store_guard.as_store_ref(), ptr.offset(), data)
    }

    fn read_ptr(&self, ptr: &Pointer) -> Result<Vec<u8>> {
        let store_guard =
            self.store_lock
                .as_ref()
                .lock()
                .map_err(|original| Error::RuntimeError {
                    msg: format!("{original}"),
                })?;

        let memory = self.memory()?;
        memory::read_ptr(
            memory,
            &store_guard.as_store_ref(),
            ptr.offset(),
            ptr.length(),
        )
    }

    fn remaining_points(&self) -> Result<u64> {
        let mut store_guard = self
            .store_lock
            .lock()
            .map_err(|original| Error::RuntimeError {
                msg: format!("{original}"),
            })?;

        match get_remaining_points(&mut store_guard.as_store_mut(), &self.instance) {
            MeteringPoints::Exhausted => Ok(0),
            MeteringPoints::Remaining(points) => Ok(points),
        }
    }

    fn consumed_points(&self) -> Result<u64> {
        Ok(self.metering_limit - self.remaining_points()?)
    }

    fn exhausted(&self) -> Result<bool> {
        match self.remaining_points()? {
            0 => Ok(true),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain_api::MockBlockchainAPI;
    use crate::provider::MockProvider;
    use wasmer::Pages;

    fn make_test_wasmer(
        wat: &str,
        memory_limit_page: u32,
        metering_limit: u64,
    ) -> Result<WasmerExecutor> {
        let code = wat::parse_str(wat).unwrap();
        let _blockchain_api = MockBlockchainAPI::new();
        let provider = Arc::new(Mutex::new(MockProvider::new()));
        WasmerExecutor::new(&code, memory_limit_page, metering_limit, provider)
    }

    #[test]
    fn test_insufficient_memory() {
        let wat = r#"
(module
    (memory $0 1)
    (export "memory" (memory $0))
)"#;

        let res = make_test_wasmer(wat, 0, 1000);
        assert!(res.is_err());
    }

    #[test]
    fn test_exported_memory() {
        let wat = r#"
(module
    (memory 4)
    (export "memory" (memory 0))
)"#;
        let wasmer = make_test_wasmer(wat, 4, 1000).unwrap();
        let mem = wasmer.instance.exports.get_memory("memory").unwrap();
        let mut guard = wasmer.store_lock.lock().unwrap();

        assert_eq!(mem.ty(&guard.as_store_mut()).minimum, Pages(4));
        assert_eq!(mem.ty(&guard.as_store_mut()).maximum, Some(Pages(4)));
    }

    #[test]
    fn test_call_no_params() {
        let wat = r#"
(module
    (type $t0 (func))
    (func $nope (type $t0))
    (export "nope" (func $nope))
    (memory $0 0)
    (export "memory" (memory $0))
)"#;

        let wasmer = make_test_wasmer(wat, 0, 1000).unwrap();
        let res = wasmer.call_function("nope", &[]);
        assert!(res.is_ok());
    }

    #[test]
    fn test_call_with_params() {
        let wat = r#"
(module
    (type $t0 (func))
    (func $add (param $param0 i32) (param $param1 i32) (result i32)
        (i32.add
            (local.get $param0)
            (local.get $param1)
        )
    )
    (export "add" (func $add))
    (memory $0 1)
    (export "memory" (memory $0))
)"#;

        let wasmer = make_test_wasmer(wat, 1, 1000).unwrap();
        let res = wasmer
            .call_function("add", &[Value::I32(1), Value::I32(2)])
            .unwrap();
        assert_eq!(res.to_vec(), vec![Value::I32(3)]);
    }
}
