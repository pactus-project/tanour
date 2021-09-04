use std::ops::Deref;

use crate::compile;
use crate::error::{Error, Result};
use crate::provider_api::ProviderAPI;
use crate::types::{Address, Bytes};
use crate::{memory, state::State};
use log::debug;
#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
#[cfg(not(feature = "cranelift"))]
use wasmer::{
    BaseTunables, Exports, ImportObject, Module, Singlepass, Store, Target, Universal, Val,
};

const PAGE_SIZE: usize = 1024 * 1024; // 1 kilobyte

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct Contract<P> {
    /// Address of the code.
    address: Address,
    /// Wasmer instance of the code
    instance: wasmer::Instance,
    /// State of the contract
    state: State<P>,
}

impl<P> Contract<P>
where
    P: ProviderAPI,
{
    pub fn new(
        provider: P,
        address: Address,
        code: &Bytes,
        memory_limit: u64,
    ) -> Result<Self> {
        let module = compile::compile(&code, memory_limit)?;

        let mut import_obj = ImportObject::new();
        let mut env_imports = Exports::new();

        import_obj.register("env", env_imports);

        let instance = wasmer::Instance::new(&module, &import_obj).map_err(|original| {
            Error::InstantiationError {
                msg: format!("{:?}", original),
            }
        })?;

        let state = State::new(provider, address, PAGE_SIZE);

        Ok(Contract {
            address,
            instance,
            state,
        })
    }

    pub fn call_process_msg_function(&self, args: &[u8]) -> Result<&[u8]> {
        //self.state.make_readonly(false);
        //self.call_function("process_msg", args)
        todo!()
    }

    /// Calls a function with the given arguments.
    fn call_function(&self, name: &str, args: &[Val]) -> Result<Box<[Val]>> {
        let vals = Vec::<Val>::with_capacity(args.len());

        let func = self
            .instance
            .exports
            .get_function(&name)
            .map_err(|original| Error::RuntimeError {
                func_name: name.to_owned(),
                msg: format!("{}", original),
            })?;

        let result = func.call(&args).map_err(|original| Error::RuntimeError {
            func_name: name.to_owned(),
            msg: format!("{}", original),
        })?;

        Ok(result)
    }
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod instance_test;
