use crate::action::Action;
use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::types::{Address, Bytes};
use crate::utils;
use crate::{compile, memory};
use log::{debug, trace};
#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
#[cfg(not(feature = "cranelift"))]
use wasmer::Singlepass;
use wasmer::{
    wasmparser::Operator, BaseTunables, CompilerConfig, Engine, Pages, Store, Target, Universal,
    WASM_PAGE_SIZE,
};
use wasmer::{Exports, Function, ImportObject, Instance as WasmerInstance, Module, Val};

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct Instance {
    /// Address of the code.
    address: Address,
    /// Wasmer instance of the code
    instance: WasmerInstance,
}

impl Instance {
    pub fn new(address: Address, code: &Bytes, memory_limit: u64) -> Result<Self> {
        let module = Instance::compile(&code, memory_limit)?;

        let mut import_obj = ImportObject::new();
        let mut env_imports = Exports::new();

        import_obj.register("env", env_imports);

        let instance = WasmerInstance::new(&module, &import_obj).map_err(|original| {
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
    /// The exported function must return exactly one result (an offset to the result Region).
    fn call_raw(&self, name: &str, args: &[&[u8]]) -> Result<Vec<u8>> {
        let mut arg_region_ptrs = Vec::<Val>::with_capacity(args.len());
        // for arg in args {
        //     let region_ptr = self.allocate(arg.len())?;
        //     instance.write_memory(region_ptr, arg)?;
        //     arg_region_ptrs.push(region_ptr.into());
        // }
        let result = self.call_function1(name, &arg_region_ptrs)?;
        // let res_region_ptr = ref_to_u32(&result)?;
        // let data = instance.read_memory(res_region_ptr, result_max_length)?;
        // // free return value in wasm (arguments were freed in wasm code)
        // instance.deallocate(res_region_ptr)?;
        //Ok(data)
        Ok(Vec::new())
    }

    /// Calls a function exported by the instance.
    /// The function is expected to return no value. Otherwise this calls errors.
    fn call_function0(&self, name: &str, args: &[Val]) -> Result<()> {
        let result = self.call_function(name, args)?;
        let expected = 0;
        let actual = result.len();
        if actual != expected {
            // return Err(Error::result_mismatch(name, expected, actual));
        }
        Ok(())
    }

    /// Calls a function exported by the instance.
    /// The function is expected to return one value. Otherwise this calls errors.
    fn call_function1(&self, name: &str, args: &[Val]) -> Result<Val> {
        let result = self.call_function(name, args)?;
        let expected = 1;
        let actual = result.len();
        if actual != expected {
            // return Err(Error::result_mismatch(name, expected, actual));
        }
        Ok(result[0].clone())
    }

    /// Calls a function with the given name and arguments.
    /// The number of return values is variable and controlled by the guest.
    /// Usually we expect 0 or 1 return values.
    fn call_function(&self, name: &str, args: &[Val]) -> Result<Box<[Val]>> {
        // Clone function before calling it to avoid dead locks
        let func = self
            .instance
            .exports
            .get_function(&name)
            .map_err(|original| Error::RuntimeError {
                func_name: name.to_owned(),
                msg: format!("{}", original),
            })?;

        func.call(args).map_err(|original| Error::RuntimeError {
            func_name: name.to_owned(),
            msg: format!("{}", original),
        })
    }
}

#[cfg(test)]
#[path = "./instance_test.rs"]
mod instance_test;
