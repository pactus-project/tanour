use crate::error::{Error, Result};
use crate::executor::Executor;
use crate::provider_api::ProviderAPI;
use crate::state::State;
use crate::wasmer;
use minicbor::{Decode, Encode};
use std::sync::{Arc, Mutex};

const PAGE_SIZE: usize = 1024 * 1024; // 1 kilobyte

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct Contract<P> {
    /// Wasm executor
    executor: Box<dyn Executor>,
    /// State of the contract
    _state: Arc<Mutex<State<P>>>,
    /// FIXME -> Why we need the buffer?
    buffer: Vec<u8>,
}

impl<P> Contract<P>
where
    P: ProviderAPI,
{
    pub fn new(provider: P, code: &[u8], memory_limit: u64) -> Result<Self> {
        let state = Arc::new(Mutex::new(State::new(provider, PAGE_SIZE)));
        let executor = wasmer::WasmerExecutor::new(code, memory_limit, state.clone())?;

        Ok(Contract {
            executor: Box::new(executor),
            _state: state,
            buffer: Vec::new(),
        })
    }

    pub fn call_process_msg<'a, E: Encode, D: Decode<'a>>(&'a mut self, msg: E) -> Result<D> {
        let data = minicbor::to_vec(msg).map_err(|original| Error::SerializationError {
            msg: format!("{}", original),
        })?;
        let size = data.len() as u32;
        let ptr = self.allocate(size)?;

        self.executor.write_ptr(ptr, &data)?;

        let region = self.executor.call_fn_3("process_msg", ptr, size)?;

        self.deallocate(ptr)?;
        self.region_to_result(region)
    }

    fn region_to_result<'a, D: Decode<'a>>(&'a mut self, pointer: u64) -> Result<D> {
        let len = (pointer >> 32) as u32;
        let ptr = (pointer & 0xFFFFFFFF) as u32;
        self.buffer = self.executor.read_ptr(ptr, len as usize)?;
        minicbor::decode(&self.buffer).map_err(|original| Error::SerializationError {
            msg: format!("{}", original),
        })
    }

    fn allocate(&self, size: u32) -> Result<u32> {
        self.executor.call_fn_2("allocate", size)
    }

    fn deallocate(&self, ptr: u32) -> Result<()> {
        self.executor.call_fn_1("deallocate", ptr)
    }
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod tests;
