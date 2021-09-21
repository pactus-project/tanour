use super::{compile, memory};
use crate::error::{Error, Result};
use crate::executor;
use crate::types::Bytes;

#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
use wasmer::Memory;
#[cfg(not(feature = "cranelift"))]
use wasmer::{BaseTunables, Exports, ImportObject, Val};

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
    pub fn new(code: &[u8], memory_limit: u64) -> Result<Self> {
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

    fn call_function(&self, name: &str, vals: &[Val]) -> Result<Box<[Val]>> {
        let func = self
            .instance
            .exports
            .get_function(&name)
            .map_err(|original| Error::RuntimeError {
                msg: format!("{}", original),
            })?;

        func.call(vals).map_err(|original| Error::RuntimeError {
            msg: format!("{}", original),
        })
    }

    fn memory(&self) -> Result<Memory> {
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
    fn call_fn_1(&self, name: &str, arg: u32) -> Result<()> {
        let val = wasmer::Val::I32(arg as i32);
        let result = self.call_function(name, &[val])?;

        match result.first() {
            Some(val) => Err(Error::RuntimeError {
                msg: format!("Invalid return value for {}", name),
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
        memory::write_ptr(&self.memory()?, ptr, data)
    }

    fn read_ptr(&self, ptr: u32, len: usize) -> Result<Vec<u8>> {
        memory::read_ptr(&self.memory()?, ptr, len)
    }
}

#[cfg(test)]
#[path = "./executor_test.rs"]
mod tests;
