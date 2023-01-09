use crate::error::{Error, Result};
use wasmer::WasmPtr;

pub fn write_ptr(
    memory: &wasmer::Memory,
    store: &wasmer::StoreRef,
    ptr: u32,
    data: &[u8],
) -> Result<()> {
    let memory_view = memory.view(&store);
    let len = data.len() as u32;
    let buf = WasmPtr::<u8>::new(ptr)
        .slice(&memory_view, len)
        .map_err(|original| Error::MemoryError {
            msg: format!("{original}"),
        })?;

    buf.write_slice(data)
        .map_err(|original| Error::MemoryError {
            msg: format!("{original}"),
        })?;

    Ok(())
}

pub fn read_ptr(
    memory: &wasmer::Memory,
    store: &wasmer::StoreRef,
    ptr: u32,
    len: u32,
) -> Result<Vec<u8>> {
    let memory_view = memory.view(&store);
    let buf = WasmPtr::<u8>::new(ptr)
        .slice(&memory_view, len)
        .map_err(|original| Error::MemoryError {
            msg: format!("{original}"),
        })?;

    let mut result = vec![0u8; len as usize];
    buf.read_slice(&mut result)
        .map_err(|original| Error::MemoryError {
            msg: format!("{original}"),
        })?;
    Ok(result)
}
