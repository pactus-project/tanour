use crate::error::{Error, Result};
use wasmer::{Array, WasmPtr};

pub fn write_ptr(memory: &wasmer::Memory, ptr: u32, data: &[u8]) -> Result<()> {
    let len = data.len() as u32;
    match WasmPtr::<u8, Array>::new(ptr).deref(&memory, 0, len) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            for i in 0..data.len() {
                cells[i].set(data[i])
            }
            Ok(())
        }
        None => Err(Error::MemoryError {
            msg: format!("Unable to write into wasm memory.",),
        }),
    }
}

