use crate::error::{Error, Result};
use crate::executor::Executor;
use crate::provider_api::ProviderAPI;
use crate::state::State;
use crate::types::{Address, Bytes};
use crate::wasmer;
use minicbor::{Decode, Encode};

const PAGE_SIZE: usize = 1024 * 1024; // 1 kilobyte

#[derive(Debug, Clone)]
pub struct ResultData {
    pub gas_left: u64,
    pub data: Vec<u8>,
}

pub struct Contract<P> {
    /// Address of the code.
    address: Address,
    /// Wasm executor
    executor: Box<dyn Executor>,
    /// State of the contract
    state: State<P>,
}

impl<P> Contract<P>
where
    P: ProviderAPI,
{
    pub fn new(provider: P, address: Address, code: &Bytes, memory_limit: u64) -> Result<Self> {
        let executor = wasmer::executor::Executor::new(code, memory_limit)?;
        let state = State::new(provider, address, PAGE_SIZE);

        Ok(Contract {
            address,
            executor: Box::new(executor),
            state,
        })
    }

    pub fn call_process_msg<'a, M: Encode>(&self, msg: M) -> Result<()> {
        //self.state.make_readonly(false);
        let mut data = [0u8; 128];
        minicbor::encode(msg, data.as_mut()).unwrap();
        let size = data.len() as u32;
        let ptr = self.allocate(size)?;

        self.executor.write_ptr(ptr, &data)?;

        let res = self.executor.call_fn_3("process_msg", ptr, size)?;
        println!("{:?}", res);
        Ok(())
    }

    fn allocate(&self, size: u32) -> Result<u32> {
        self.executor.call_fn_2("allocate", size)
    }
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod tests;
