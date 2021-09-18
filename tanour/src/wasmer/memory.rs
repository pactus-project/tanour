use crate::error::{Error, Result};
use crate::region::Region;
use wasmer::{Array, ValueType, WasmPtr};

struct Memory {
    memory: wasmer::Memory,
}

unsafe impl ValueType for Region {}

impl Memory {
    pub fn new(memory: wasmer::Memory) -> Self {
        Self { memory }
    }

    /// Expects a (fixed size) Region struct at ptr, which is read. This links to the
    /// memory region, which is copied in the second step.
    /// Errors if the length of the region exceeds `max_length`.
    pub fn read_region(&self, ptr: u32, max_length: usize) -> Result<Vec<u8>> {
        let region = self.get_region(ptr)?;

        match WasmPtr::<u8, Array>::new(region.offset).deref(&self.memory, 0, region.length) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            let len = region.length as usize;
            let mut result = vec![0u8; len];
            for i in 0..len {
                result[i] = cells[i].get();
            }
            Ok(result)
        }
        None => Err(Error::MemoryError{msg: format!(
            "Tried to access memory of region {:?} in wasm memory of size {} bytes. This typically happens when the given Region pointer does not point to a proper Region struct.",
            region,
            self.memory.size().bytes().0
        )})
    }
    }

    /// A prepared and sufficiently large memory Region is expected at ptr that points to pre-allocated memory.
    ///
    /// Returns number of bytes written on success.
    pub fn write_region(&self, ptr: u32, data: &[u8]) -> Result<()> {
        let mut region = self.get_region(ptr)?;

        let region_capacity = region.capacity as usize;
        if data.len() > region_capacity {
            return Err(Error::MemoryError {
                msg: format!(
                    "Region is too small to write {} bytes: {:?}",
                    data.len(),
                    region
                ),
            });
        }
        match WasmPtr::<u8, Array>::new(region.offset).deref(&self.memory, 0, region.capacity) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            for i in 0..data.len() {
                cells[i].set(data[i])
            }
            region.length = data.len() as u32;
            self.set_region(ptr, region)?;
            Ok(())
        },
        None => Err(Error::MemoryError{msg: format!(
            "Tried to access memory of region {:?} in wasm memory of size {} bytes. This typically happens when the given Region pointer does not point to a proper Region struct.",
            region,
            self.memory.size().bytes().0
        )}),
    }
    }

    /// Reads in a Region at ptr in wasm memory and returns a copy of it
    fn get_region(&self, ptr: u32) -> Result<Region> {
        let wptr = WasmPtr::<Region>::new(ptr);
        match wptr.deref(&self.memory) {
            Some(cell) => {
                let region = cell.get();
                region.validate()?;
                Ok(region)
            }
            None => Err(Error::MemoryError {
                msg: format!("Could not dereference this pointer to a Region: {:?}", ptr),
            }),
        }
    }

    /// Overrides a Region at ptr in wasm memory with data
    fn set_region(&self, ptr: u32, data: Region) -> Result<()> {
        let wptr = WasmPtr::<Region>::new(ptr);

        match wptr.deref(&self.memory) {
            Some(cell) => {
                cell.set(data);
                Ok(())
            }
            None => Err(Error::MemoryError {
                msg: format!("Could not dereference this pointer to a Region: {:?}", ptr),
            }),
        }
    }
}
