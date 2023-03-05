use crate::blockchain_api::BlockchainAPI;
use crate::error::Result;
use crate::executor::Executor;
use crate::memory::Pointer;
use crate::provider::ProviderAdaptor;
use crate::{wasmer, Address};

use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

// TODO: rename me, it is confusing with ExecuteParams
#[derive(Debug)]
pub struct Params {
    pub memory_limit_page: u32,
    pub metering_limit: u64,
}

pub struct Contract {
    // Wasm executor
    executor: Box<dyn Executor>,
    // State of the contract
    _state: Arc<Mutex<ProviderAdaptor>>,
    // Contract's address
    _address: Address,
}

impl Contract {
    pub fn new(
        api: Box<dyn BlockchainAPI>,
        address: &Address,
        code: &[u8],
        params: Params,
    ) -> Result<Self> {
        let provider = Arc::new(Mutex::new(ProviderAdaptor::new(api)?));
        let executor = wasmer::WasmerExecutor::new(
            code,
            params.memory_limit_page,
            params.metering_limit,
            provider.clone(),
        )?;

        Ok(Contract {
            executor: Box::new(executor),
            _state: provider,
            _address: *address,
        })
    }

    fn call_exported_fn(&mut self, fname: &str, data: &[u8]) -> Result<Vec<u8>> {
        let size = data.len() as u32;
        let ptr_64 = self.allocate(size)?;
        let ptr = Pointer::from_u64(ptr_64);
        self.executor.write_ptr(&ptr, data)?;

        let res_ptr_64 = self.executor.call_fn_1(fname, ptr_64)?;
        self.deallocate(ptr_64)?;

        // Decoding result (result to pointer)
        let res_ptr = Pointer::from_u64(res_ptr_64);
        self.executor.read_ptr(&res_ptr)
    }

    pub fn call_instantiate(&mut self, encoded_arg: &[u8]) -> Result<Vec<u8>> {
        self.call_exported_fn("instantiate", encoded_arg)
    }

    pub fn call_process(&mut self, encoded_arg: &[u8]) -> Result<Vec<u8>> {
        self.call_exported_fn("process", encoded_arg)
    }

    pub fn call_query(&mut self, encoded_arg: &[u8]) -> Result<Vec<u8>> {
        self.call_exported_fn("query", encoded_arg)
    }

    fn allocate(&self, size: u32) -> Result<u64> {
        self.executor.call_fn_2("allocate", size)
    }

    fn deallocate(&self, ptr_64: u64) -> Result<()> {
        self.executor.call_fn_0("deallocate", ptr_64)
    }

    pub fn remaining_points(&self) -> Result<u64> {
        self.executor.remaining_points()
    }

    pub fn consumed_points(&self) -> Result<u64> {
        self.executor.consumed_points()
    }

    pub fn exhausted(&self) -> Result<bool> {
        self.executor.exhausted()
    }
}
