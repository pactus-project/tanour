use super::compile;
use super::limiting_tunables;
use crate::error::{Error, Result};
use crate::executor;
use crate::region::Region;
use crate::types::Bytes;
use log::debug;
use std::convert::{TryFrom, TryInto};
use wasmer::Memory;

#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
#[cfg(not(feature = "cranelift"))]
use wasmer::{BaseTunables, Exports, ImportObject, Module, Singlepass, Store, Target, Universal};

const PAGE_SIZE: usize = 1024 * 1024; // 1 kilobyte

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct Executor {
    /// Wasmer instance of the code
    instance: wasmer::Instance,
}

impl Executor {
    pub fn new(code: &Bytes, memory_limit: u64) -> Result<Self> {
        let module = compile::compile(&code, memory_limit)?;

        let mut import_obj = ImportObject::new();
        let mut env_imports = Exports::new();

        import_obj.register("zarb", env_imports);

        let instance = wasmer::Instance::new(&module, &import_obj).map_err(|original| {
            Error::InstantiationError {
                msg: format!("{:?}", original),
            }
        })?;

        Ok(Executor { instance })
    }

    fn memory(&self) -> Result<Memory> {
        // Check what happened
        let mut memories: Vec<Memory> = self
            .instance
            .exports
            .iter()
            .memories()
            .map(|pair| pair.1.clone())
            .collect();
        if memories.len() != 1 {
            Err(Error::RuntimeError {
                msg: "Invalid memory map".to_owned(),
            })
        } else {
            Ok(memories.pop().unwrap())
        }
    }
}

impl executor::Executor for Executor {
    fn call_function(&self, name: &str, args_ptr: &Region) -> Result<Option<Region>> {
        let func = self
            .instance
            .exports
            .get_function(&name)
            .map_err(|original| Error::RuntimeError {
                msg: format!("{}", original),
            })?;

        let mut wasmer_args = Vec::new();
        for arg in args {
            wasmer_args.push(arg.into());
        }

        let result = func
            .call(&wasmer_args)
            .map_err(|original| Error::RuntimeError {
                msg: format!("{}", original),
            })?;

        match result.first() {
            Some(val) => Ok(Some(val.try_into()?)),
            None => Ok(None),
        }
    }
}

impl From<&Value> for wasmer::Val {
    fn from(val: &Value) -> Self {
        match val {
            Value::I32(i32) => wasmer::Val::I32(*i32),
            Value::I64(i64) => wasmer::Val::I64(*i64),
        }
    }
}

impl TryFrom<&wasmer::Val> for Value {
    type Error = Error;

    fn try_from(val: &wasmer::Val) -> Result<Self> {
        match val {
            wasmer::Val::I32(i32) => Ok(Value::I32(*i32)),
            wasmer::Val::I64(i64) => Ok(Value::I64(*i64)),
            _ => Err(Error::RuntimeError {
                msg: "Invalid wasmer value".to_string(),
            }),
        }
    }
}
