use crate::action::Action;
use crate::compile;
use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::types::{Address, Bytes};
use crate::{memory, storage::Storage};
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

pub struct Contract<P> {
    /// Address of the code.
    address: Address,
    /// Wasmer instance of the code
    instance: wasmer::Instance,
    /// storage handler of the contract
    storage: Storage<P>,
    ///
    provider: P,
}

impl<P> Contract<P>
where
    P: Provider,
{
    pub fn instantiate(
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

        let storage = Storage::new(provider);

        Ok(Contract {
            address,
            instance,
            storage,
            provider,
        })
    }

    pub fn execute(&self, _action: Action) -> Result<()> {
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
#[path = "./contract_test.rs"]
mod instance_test;
