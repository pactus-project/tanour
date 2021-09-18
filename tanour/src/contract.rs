use crate::error::{Error, Result};
use crate::executor::Executor;
use crate::provider_api::ProviderAPI;
use crate::state::State;
use crate::types::{Address, Bytes};
use crate::wasmer;
use minicbor::Encode;

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

    pub fn call_process_msg<E: Encode>(&self, msg: E) -> Result<&[u8]> {
        //self.state.make_readonly(false);
        let mut arg = [0u8; 128];
        minicbor::encode(msg, arg.as_mut()).unwrap();
        let size = arg.len();
        let ptr = self.allocate(size as i32)?;

        let mut vec = unsafe { Vec::from_raw_parts(ptr as *mut u8, size, size) };
        vec.clone_from_slice(arg);
        let mut vals = Vec::<Value>::with_capacity(2);
        vals[0] = Value::I32(ptr as i32);
        vals[1] = Value::I32(size as i32);

        let res = self.call_function("process_msg", &vals)?;
        println!("{:?}", res);
        Ok(&[0])
    }

    fn allocate(&self, size: i32) -> Result<u32> {
        let arg = Value::I32(size);
        let res = self.call_function("allocate", &[arg])?;
        if let Value::I32(ptr) = res.unwrap() {
            return Ok(ptr as u32);
        }

        Err(Error::RuntimeError {
            msg: "invalid allocation".to_string(),
        })
    }

    /// Calls a function with the given arguments.
    fn call_function(&self, name: &str, args: &[Value]) -> Result<Option<Value>> {
        self.executor.call_function(name, args)
    }
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
