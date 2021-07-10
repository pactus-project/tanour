use crate::action::Action;
use crate::error::{Error, Result};
use crate::memory;
use crate::provider::Provider;
use crate::types::{Address, Bytes};
use log::debug;
#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
#[cfg(not(feature = "cranelift"))]
use wasmer::{
    BaseTunables, Exports, ImportObject, Module, Singlepass, Store, Target, Universal, Val,
};

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct Instance {
    /// Address of the code.
    address: Address,
    /// Wasmer instance of the code
    instance: wasmer::Instance,
}

impl Instance {
    pub fn new(address: Address, code: &Bytes, memory_limit: u64) -> Result<Self> {
        let module = Instance::compile(&code, memory_limit)?;

        let mut import_obj = ImportObject::new();
        let mut env_imports = Exports::new();

        import_obj.register("env", env_imports);

        let instance = wasmer::Instance::new(&module, &import_obj).map_err(|original| {
            Error::InstantiationError {
                msg: format!("{:?}", original),
            }
        })?;

        Ok(Instance { address, instance })
    }

    /// Compiles a given Wasm bytecode into a module.
    /// The given memory limit (in bytes) is used when memories are created.
    fn compile(code: &[u8], memory_limit: u64) -> Result<Module> {
        let gas_limit = 0;
        let mut config;

        #[cfg(feature = "cranelift")]
        {
            config = Cranelift::default();
        };

        #[cfg(not(feature = "cranelift"))]
        {
            config = Singlepass::default();
        };

        let engine = Universal::new(config).engine();
        let base = BaseTunables::for_target(&Target::default());
        let tunables =
            memory::LimitingTunables::new(base, memory::limit_to_pages(memory_limit as usize));
        let store = Store::new_with_tunables(&engine, tunables);

        let module = Module::new(&store, code).map_err(|original| Error::CompileError {
            msg: format!("{}", original),
        })?;

        Ok(module)
    }

    pub fn execute(&self, provider: &mut dyn Provider, action: Action) -> Result<()> {
        Ok(())
    }

    /// Calls a function with the given arguments.
    fn call_function(&self, name: &str, args: &[&[u8]]) -> Result<&[u8]> {
        let vals = Vec::<Val>::with_capacity(args.len());

        let func = self
            .instance
            .exports
            .get_function(&name)
            .map_err(|original| Error::RuntimeError {
                func_name: name.to_owned(),
                msg: format!("{}", original),
            })?;

        let result = func.call(&vals).map_err(|original| Error::RuntimeError {
            func_name: name.to_owned(),
            msg: format!("{}", original),
        })?;

        debug!("result: {:?}", result);

        Ok(&[0])
    }
}

#[cfg(test)]
#[path = "./instance_test.rs"]
mod instance_test;
