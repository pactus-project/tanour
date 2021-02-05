use crate::action::ActionParams;
use crate::error::Error;
use crate::log_entry::LogEntry;
use crate::schedule::Schedule;
use crate::state::State;
use crate::types::{Address, Bytes, Hash32};
use log::trace;
use wasmi::LittleEndianConvert;
use wasmi::{MemoryRef, RuntimeArgs, RuntimeValue};


macro_rules! set_seal_value {
    ($name:literal, $self:expr ,$dest_ptr:expr, $len_ptr:expr, $buf:expr) => {{
        trace!("{}: {:x?}", $name, $buf);

        let len = $self
            .memory
            .get_value::<u32>($len_ptr)
            .expect(&format!("{} len_ptr should be valid", $name));

        if (len as usize) < $buf.len() {
            panic!("{} input is {} buffer is {}", $name, $buf.len(), len);
        }

        if let Err(e) = $self.memory.set($dest_ptr, $buf) {
            panic!("{}: {}", $name, e);
        }

        $self.memory
            .set_value($len_ptr, $buf.len() as u32)
            .expect(&format!("{} len_ptr should be valid", $name));
    }};
}

pub enum ReturnCode {
    /// API call successful.
    Success = 0,
    /// The called function trapped and has its state changes reverted.
    /// In this case no output buffer is returned.
    CalleeTrapped = 1,
    /// The called function ran to completion but decided to revert its state.
    /// An output buffer is returned when one was supplied.
    CalleeReverted = 2,
    /// The passed key does not exist in storage.
    KeyNotFound = 3,
    /// Transfer failed because it would have brought the sender's total balance below the
    /// subsistence threshold.
    BelowSubsistenceThreshold = 4,
    /// Transfer failed for other reasons. Most probably reserved or locked balance of the
    /// sender prevents the transfer.
    TransferFailed = 5,
    /// The newly created contract is below the subsistence threshold after executing
    /// its constructor.
    NewContractNotFunded = 6,
    /// No code could be found at the supplied code hash.
    CodeNotFound = 7,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable = 8,
}

pub struct Runtime<'a> {
    schedule: &'a Schedule,
    gas_counter: u64,
    gas_limit: u64,
    params: &'a ActionParams,
    memory: MemoryRef,
    result: Vec<u8>,
    state: &'a mut State<'a>,
    logs: Vec<LogEntry>,
}

impl<'a> Runtime<'a> {
    /// New runtime for wasm contract with specified params
    pub fn new(
        params: &'a ActionParams,
        schedule: &'a Schedule,
        state: &'a mut State<'a>,
        memory: MemoryRef,
        gas_limit: u64,
    ) -> Self {
        Runtime {
            schedule: schedule,
            gas_counter: 0,
            gas_limit: gas_limit,
            memory: memory,
            params: params,
            state: state,
            logs: Vec::new(),
            result: Vec::new(),
        }
    }



    fn read_memory_at_hash32(&self, ptr: u32) -> Result<Hash32, Error> {
        let mut buf = [0u8; 32];
        self.memory.get_into(ptr, &mut buf[..])?;

        Ok(Hash32::from_slice(&buf[..]))
    }

    /// Read from the storage to wasm memory
    pub fn seal_get_storage(&mut self, args: RuntimeArgs) -> Result<RuntimeValue, Error> {
        let key_ptr = args.nth_checked(0)?;
        let val_ptr: u32 = args.nth_checked(1)?;
        let val_len_ptr: u32 = args.nth_checked(2)?;

        let key = self.read_memory_at_hash32(key_ptr)?;

        match self.state.storage_read(self.params.address, key) {
            Ok(val) => {
                self.memory.set(val_ptr, &val[..])?;
                self.memory.set_value(val_len_ptr, val.len() as u32)?;
                Ok(RuntimeValue::from(ReturnCode::Success as u32))
            }
            Err(Error::KeyNotFound) => Ok(RuntimeValue::from(ReturnCode::KeyNotFound as u32)),
            Err(e) => Err(e),
        }
    }

    // Write to storage from wasm memory
    pub fn seal_set_storage(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        let key_ptr = args.nth_checked(0)?;
        let val_ptr: u32 = args.nth_checked(1)?;
        let val_len_ptr: u32 = args.nth_checked(2)?;

        let key = self.read_memory_at_hash32(key_ptr)?;
        let val_len: u32 = self.memory.get_value(val_len_ptr)?;
        let val = self.memory.get(val_ptr, val_len as usize)?;

        self.state.set_storage(self.params.address, key, val)?;

        Ok(())
    }

    pub fn seal_clear_storage(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        Ok(())
    }

    pub fn seal_return(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        let buf_ptr: u32 = args.nth_checked(0)?;
        let buf_ptr_len: u32 = args.nth_checked(1)?;

        let len: u32 = self.memory.get_value(buf_ptr_len)?;
        self.result = self.memory.get(buf_ptr, len as usize)?;

        Err(Error::Return)
    }

    pub fn seal_value_transferred(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        let buf_ptr: u32 = args.nth_checked(0)?;
        let len_ptr: u32 = args.nth_checked(1)?;

        let v : u128 = 0;
        let scratch =v.to_le_bytes().to_vec();

        set_seal_value!("seal_value_transferred", self, buf_ptr, len_ptr, &scratch);

        Ok(())
    }

    /// Write input bytes to the memory location using the passed pointer
    fn seal_input(&mut self, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Error> {
        let buf_ptr: u32 = args.nth_checked(0)?;
        let len_ptr: u32 = args.nth_checked(1)?;

        let input = &self.params.args;

        let len = self
                    .memory
                    .get_value::<u32>(len_ptr)
                    .expect("seal_input len_ptr should be valid");

                if (len as usize) < input.len() {
                    panic!("input is {} seal_input buffer {}", input.len(), len);
                }

                if let Err(e) = self.memory.set(buf_ptr, &input) {
                    panic!("seal_input: {}", e);
                }

                self
                    .memory
                    .set_value(len_ptr, input.len() as u32)
                    .expect("seal_input len_ptr should be valid");


        Ok(None)
    }

    ///	Signature: `fn blocknumber() -> i64`
    pub fn seal_block_number(&mut self) -> Result<(), Error> {
        //Ok(RuntimeValue::from(self.state.block_number()))
        Ok(())
    }

    pub fn seal_address(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        let address = self.params.address;
        self.return_address_ptr(args.nth_checked(0)?, address)
    }

    pub fn seal_caller(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        let sender = self.params.sender;
        self.return_address_ptr(args.nth_checked(0)?, sender)
    }

    /// General gas charging extern.
    fn seal_gas(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        Ok(())
        // let amount: u32 = args.nth_checked(0)?;
        // if self.charge_gas(amount as u64) {
        //     Ok(())
        // } else {
        //     Err(Error::GasLimit.into())
        // }
    }

    /// Query current gas left for execution
    pub fn seal_gas_left(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn seal_hash_blake2_256(&mut self, args: RuntimeArgs) -> Result<(), Error> {
        Ok(())
    }

    /// Query current gas left for execution
    pub fn gas_left(&self) -> Result<u64, Error> {
        if self.gas_counter > self.gas_limit {
            return Err(Error::InvalidGasState);
        }
        Ok(self.gas_limit - self.gas_counter)
    }

    /// Return currently used schedule
    pub fn schedule(&self) -> &Schedule {
        self.schedule
    }

    /// Destroy the runtime, returning currently recorded result of the execution
    pub fn into_result(&self) -> Vec<u8> {
        self.result.clone()
    }

    fn return_address_ptr(&mut self, ptr: u32, val: Address) -> Result<(), Error> {
        // self.charge(|schedule| schedule.wasm().static_address as u64)?;
        self.memory.set(ptr, val.as_bytes())?;
        Ok(())
    }

    fn return_u64_ptr(&mut self, ptr: u32, val: u64) -> Result<(), Error> {
        let ret = val.to_be_bytes();
        //self.charge(|schedule| schedule.wasm().static_u256 as u64)?;
        self.memory.set(ptr, &ret)?;
        Ok(())
    }

    pub fn init_code(&mut self, address: Address, code: Vec<u8>) {
        self.state.init_code(address, code);
    }

    pub fn update_state(&mut self) -> Result<(), Error> {
        self.state.update_state()
    }

}

mod ext_impl {

    use crate::functions::func_id::*;
    use wasmi::{Externals, RuntimeArgs, RuntimeValue, Trap};

    macro_rules! void {
		{ $e: expr } => { { $e?; Ok(None) } }
    }

    macro_rules! some {
		{ $e: expr } => { { Ok(Some($e?)) } }
	}

    impl<'a> Externals for super::Runtime<'a> {
        fn invoke_index(
            &mut self,
            index: usize,
            args: RuntimeArgs,
        ) -> Result<Option<RuntimeValue>, Trap> {
            match index {
                GET_STORAGE_FUNC_ID => some!(self.seal_get_storage(args)),
                SET_STORAGE_FUNC_ID => void!(self.seal_set_storage(args)),
                CLEAR_STORAGE_FUNC_ID => void!(self.seal_clear_storage(args)),
                VALUE_TRANSFERRED_FUNC_ID => void!(self.seal_value_transferred(args)),
                INPUT_FUNC_ID => void!(self.seal_input(args)),
                ADDRESS_FUNC_ID => void!(self.seal_address(args)),
                CALLER_FUNC_ID => void!(self.seal_caller(args)),
                RETURN_FUNC_ID => void!(self.seal_return(args)),
                GAS_FUNC_ID => void!(self.seal_gas(args)),
                GAS_LEFT_FUNC_ID => void!(self.seal_gas_left()),
                BLOCK_NUMBER_FUNC_ID => void!(self.seal_block_number()),
                HASH_BLAKE2_256_FUNC_ID => void!(self.seal_hash_blake2_256(args)),

                _ => panic!("env module doesn't provide function at index {}", index),
            }
        }
    }
}
