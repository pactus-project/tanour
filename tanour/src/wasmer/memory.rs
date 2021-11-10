use crate::error::{Error, Result};
use wasmer::{Array, WasmPtr};

pub fn write_ptr(memory: &wasmer::Memory, ptr: u32, data: &[u8]) -> Result<()> {
    let len = data.len() as u32;
    match WasmPtr::<u8, Array>::new(ptr).deref(memory, 0, len) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            for i in 0..data.len() {
                cells[i].set(data[i])
            }
            Ok(())
        }
        None => Err(Error::MemoryError {
            msg: "Unable to write into wasm memory.".to_string(),
        }),
    }
}

pub fn read_ptr(memory: &wasmer::Memory, ptr: u32, len: u32) -> Result<Vec<u8>> {
    match WasmPtr::<u8, Array>::new(ptr).deref(memory, 0, len) {
        Some(cells) => {
            // In case you want to do some premature optimization, this shows how to cast a `&'mut [Cell<u8>]` to `&mut [u8]`:
            // https://github.com/wasmerio/wasmer/blob/0.13.1/lib/wasi/src/syscalls/mod.rs#L79-L81
            let mut result = vec![0u8; len as usize];
            for i in 0..len as usize {
                result[i] = cells[i].get();
            }
            Ok(result)
        }
        None => Err(Error::MemoryError {
            msg: "Unable to read from wasm memory.".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::state::MockStateTrait;
    use crate::wasmer::env::tests::make_test_env;
    use crate::ONE_MB;
    use wasmer::ImportObject;
    use wasmer::Pages;

    #[test]
    fn test_exported_memory() {
        let wat = r#"
(module
    (memory 4)
    (export "memory" (memory 0))
)"#;
        let (env, _instance) = make_test_env(
            wat,
            ONE_MB,
            1000,
            &ImportObject::new(),
            MockStateTrait::new(),
        );
        let mem = env.memory().unwrap();

        assert_eq!(mem.ty().minimum, Pages(4));
        assert_eq!(mem.ty().maximum, Some(Pages(16)));
    }
}
