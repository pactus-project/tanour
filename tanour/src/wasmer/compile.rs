use super::limiting_tunables;
use crate::error::{Error, Result};
use log::debug;
use wasmer::Singlepass;
use wasmer::{BaseTunables, Module, Store, Target, Universal};

/// Compiles a given Wasm bytecode into a module.
/// The given memory limit (in bytes) is used when memories are created.
pub fn compile(code: &[u8], memory_limit: u64) -> Result<Module> {
    debug!("compiling the code");
    let config = Singlepass::default();
    let engine = Universal::new(config).engine();
    let base = BaseTunables::for_target(&Target::default());
    let tunables = limiting_tunables::LimitingTunables::new(
        base,
        limiting_tunables::limit_to_pages(memory_limit as usize),
    );
    let store = Store::new_with_tunables(&engine, tunables);

    let module = Module::new(&store, code).map_err(|original| Error::CompileError {
        msg: format!("{}", original),
    })?;

    Ok(module)
}
