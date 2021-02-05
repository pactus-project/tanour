use crate::wasm_cost::WasmCosts;
use std::cell::RefCell;
use wasmi::{
    self, memory_units, Error, FuncInstance, FuncRef, MemoryDescriptor, MemoryInstance, MemoryRef,
    Signature,
};

pub mod func_id {
    pub const SET_STORAGE_FUNC_ID: usize = 0x01;
    pub const GET_STORAGE_FUNC_ID: usize = 0x02;
    pub const CLEAR_STORAGE_FUNC_ID: usize = 0x03;
    pub const VALUE_TRANSFERRED_FUNC_ID: usize = 0x04;
    pub const INPUT_FUNC_ID: usize = 0x05;
    pub const ADDRESS_FUNC_ID: usize = 0x06;
    pub const CALLER_FUNC_ID: usize = 0x07;
    pub const RETURN_FUNC_ID: usize = 0x08;
    pub const GAS_FUNC_ID: usize = 0x09;
    pub const GAS_LEFT_FUNC_ID: usize = 0x0A;
    pub const BLOCK_NUMBER_FUNC_ID: usize = 0x0B;
    pub const HASH_BLAKE2_256_FUNC_ID: usize = 0x0C;
}

pub mod func_sig {
    use wasmi::ValueType::*;
    use wasmi::{self, ValueType};

    pub struct StaticSignature(pub &'static [ValueType], pub Option<ValueType>);

    pub const GET_STORAGE_SIG: StaticSignature = StaticSignature(&[I32, I32, I32], Some(I32));
    pub const SET_STORAGE_SIG: StaticSignature = StaticSignature(&[I32, I32, I32], None);
    pub const CLEAR_STORAGE_SIG: StaticSignature = StaticSignature(&[I32], None);
    pub const VALUE_TRANSFERRED_SIG: StaticSignature = StaticSignature(&[I32, I32], None);
    pub const INPUT_SIG: StaticSignature = StaticSignature(&[I32, I32], None);
    pub const ADDRESS_SIG:StaticSignature = StaticSignature(&[I32, I32], None);
    pub const CALLER_SIG:StaticSignature = StaticSignature(&[I32, I32], None);
    pub const RETURN_SIG: StaticSignature = StaticSignature(&[I32, I32, I32], None);
    pub const GAS_SIG: StaticSignature = StaticSignature(&[I32], None);
    pub const GAS_LEFT_SIG: StaticSignature = StaticSignature(&[], Some(I32));
    pub const BLOCK_NUMBER_SIG: StaticSignature = StaticSignature(&[], Some(I64));
    pub const HASH_BLAKE2_256_SIG: StaticSignature = StaticSignature(&[I32, I32, I32], None);

    impl Into<wasmi::Signature> for StaticSignature {
        fn into(self) -> wasmi::Signature {
            wasmi::Signature::new(self.0, self.1)
        }
    }
}

fn host(func_sig: func_sig::StaticSignature, func_id: usize) -> FuncRef {
    FuncInstance::alloc_host(func_sig.into(), func_id)
}

/// Import resolver for wasmi
/// Maps all functions that runtime support to the corresponding contract import
/// entries.
/// Also manages initial memory request from the runtime.
pub struct ImportResolver {
    max_memory: u32,
    memory: RefCell<Option<MemoryRef>>,
}

impl ImportResolver {
    /// New import resolver with specifed maximum amount of inital memory (in wasm pages = 64kb)
    pub fn with_limit(max_memory: u32, schedule: &WasmCosts) -> ImportResolver {
        ImportResolver {
            max_memory: max_memory,
            memory: RefCell::new(None),
        }
    }

    /// Returns memory that was instantiated during the contract module
    /// start. If contract does not use memory at all, the dummy memory of length (0, 0)
    /// will be created instead. So this method always returns memory instance
    /// unless errored.
    pub fn memory_ref(&self) -> MemoryRef {
        {
            let mut mem_ref = self.memory.borrow_mut();
            if mem_ref.is_none() {
                *mem_ref = Some(
                    MemoryInstance::alloc(memory_units::Pages(0), Some(memory_units::Pages(0)))
                        .expect("Memory allocation (0, 0) should not fail; qed"),
                );
            }
        }

        self.memory
            .borrow()
            .clone()
            .expect("it is either existed or was created as (0, 0) above; qed")
    }

    /// Returns memory size module initially requested
    pub fn memory_size(&self) -> Result<u32, Error> {
        Ok(self.memory_ref().current_size().0 as u32)
    }
}

impl wasmi::ModuleImportResolver for ImportResolver {
    fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
        let func_ref = match field_name {
            "seal_get_storage" => host(func_sig::GET_STORAGE_SIG, func_id::GET_STORAGE_FUNC_ID),
            "seal_set_storage" => host(func_sig::SET_STORAGE_SIG, func_id::SET_STORAGE_FUNC_ID),
            "seal_clear_storage" => host(func_sig::CLEAR_STORAGE_SIG, func_id::CLEAR_STORAGE_FUNC_ID),
            "seal_value_transferred" => host(
                func_sig::VALUE_TRANSFERRED_SIG,
                func_id::VALUE_TRANSFERRED_FUNC_ID,
            ),
            "seal_input" => host(func_sig::INPUT_SIG, func_id::INPUT_FUNC_ID),
            "seal_address" => host(func_sig::ADDRESS_SIG, func_id::ADDRESS_FUNC_ID),
            "seal_caller" => host(func_sig::CALLER_SIG, func_id::CALLER_FUNC_ID),
            "seal_return" => host(func_sig::RETURN_SIG, func_id::RETURN_FUNC_ID),
            "gas" => host(func_sig::GAS_SIG, func_id::GAS_FUNC_ID),
            "seal_gas_left" => host(func_sig::GAS_LEFT_SIG, func_id::GAS_LEFT_FUNC_ID),
            "seal_block_number" => host(func_sig::BLOCK_NUMBER_SIG, func_id::BLOCK_NUMBER_FUNC_ID),
            "seal_hash_blake2_256" => host(func_sig::HASH_BLAKE2_256_SIG, func_id::HASH_BLAKE2_256_FUNC_ID),
            _ => {
                return Err(wasmi::Error::Instantiation(format!(
                    "Export {} not found",
                    field_name
                )))
            }
        };

        Ok(func_ref)
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        descriptor: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        if field_name == "memory" {
            let effective_max = descriptor.maximum().unwrap_or(self.max_memory + 1);
            if descriptor.initial() > self.max_memory || effective_max > self.max_memory {
                Err(Error::Instantiation(
                    "Module requested too much memory".to_owned(),
                ))
            } else {
                let mem = MemoryInstance::alloc(
                    memory_units::Pages(descriptor.initial() as usize),
                    descriptor
                        .maximum()
                        .map(|x| memory_units::Pages(x as usize)),
                )?;
                *self.memory.borrow_mut() = Some(mem.clone());
                Ok(mem)
            }
        } else {
            Err(Error::Instantiation(
                "Memory imported under unknown name".to_owned(),
            ))
        }
    }
}
