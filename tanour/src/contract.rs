use crate::blockchain_api::BlockchainAPI;
use crate::error::{Error, Result};
use crate::executor::Executor;
use crate::memory::Pointer;
use crate::provider::Provider;
use crate::storage_file::StorageFile;
use crate::{wasmer, Address};
use minicbor::{Decode, Encode};

use std::path::Path;
use std::sync::{Arc, Mutex};

const PAGE_SIZE: usize = 1024 * 1024; // 1 kilobyte

#[derive(Debug)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Params {
    pub memory_limit_page: u32,
    pub metering_limit: u64,
    pub storage_path: String,
}

pub struct Contract {
    /// Wasm executor
    executor: Box<dyn Executor>,
    /// State of the contract
    _state: Arc<Mutex<Provider>>,
    /// internal buffer for decoding messages
    buffer: Vec<u8>,
    /// Contract's address
    address: Address,
}

impl Contract {
    pub fn create(
        blockchain_api: Box<dyn BlockchainAPI>,
        address: &Address,
        storage_size_in_mb: u32,
        code: &[u8],
        params: Params,
    ) -> Result<Self> {
        let file_path = Path::new(&params.storage_path)
            .join(format!("{}.storage", crate::address_to_hex(address)));
        let storage_file = StorageFile::create(file_path.to_str().unwrap(), storage_size_in_mb)?;
        let state = Arc::new(Mutex::new(Provider::new(blockchain_api, storage_file)));
        let executor = wasmer::WasmerExecutor::new(
            code,
            params.memory_limit_page,
            params.metering_limit,
            state.clone(),
        )?;

        Ok(Contract {
            executor: Box::new(executor),
            _state: state,
            buffer: Vec::new(),
            address: *address,
        })
    }

    // pub fn load(
    //     provider: P,
    //     address: &Address,
    //     params: Params,
    // ) -> Result<Self> {

    //     let state = Arc::new(Mutex::new(State::new(provider, PAGE_SIZE)));
    //     let executor =
    //         wasmer::WasmerExecutor::new(code, memory_limit_page, metering_limit, state.clone())?;

    //     Ok(Contract {
    //         executor: Box::new(executor),
    //         _state: state,
    //         buffer: Vec::new(),
    //         address: address.clone(),
    //     })
    // }

    fn call_exported_fn<'a, E: Encode<()>, D: Decode<'a, ()>>(
        &'a mut self,
        msg: E,
        fname: &str,
    ) -> Result<D> {
        let param_data = minicbor::to_vec(msg).map_err(|original| Error::SerializationError {
            msg: format!("{original}"),
        })?;
        let size = param_data.len() as u32;
        let ptr_64 = self.allocate(size)?;
        let ptr = Pointer::from_u64(ptr_64);
        self.executor.write_ptr(&ptr, &param_data)?;

        let res_ptr_64 = self.executor.call_fn_1(fname, ptr_64)?;
        self.deallocate(ptr_64)?;

        // Decoding result (result to pointer)
        let res_ptr = Pointer::from_u64(res_ptr_64);
        self.buffer = self.executor.read_ptr(&res_ptr)?;
        minicbor::decode(&self.buffer).map_err(|original| Error::SerializationError {
            msg: format!("{original}"),
        })
    }

    pub fn call_instantiate<'a, E: Encode<()>, D: Decode<'a, ()>>(
        &'a mut self,
        msg: E,
    ) -> Result<D> {
        self.call_exported_fn(msg, "instantiate")
    }

    pub fn call_process<'a, E: Encode<()>, D: Decode<'a, ()>>(&'a mut self, msg: E) -> Result<D> {
        self.call_exported_fn(msg, "process")
    }

    pub fn call_query<'a, E: Encode<()>, D: Decode<'a, ()>>(&'a mut self, msg: E) -> Result<D> {
        self.call_exported_fn(msg, "query")
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
