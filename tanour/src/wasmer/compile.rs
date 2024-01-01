use super::limiting_tunables::LimitingTunables;
use crate::error::{Error, Result};
use log::debug;
use std::sync::Arc;
use wasmer::{
    wasmparser::Operator, BaseTunables, CompilerConfig, EngineBuilder, Module, NativeEngineExt,
    Pages, Singlepass, Store, Target,
};
use wasmer_middlewares::Metering;

/// Compiles a given Wasm bytecode into a module.
/// The given memory limit (in bytes) is used when memories are created.
pub fn compile(
    code: &[u8],
    memory_limit_page: u32,
    metering_limit: u64,
) -> Result<(Module, Store)> {
    debug!("compiling the code");
    let mut config = Singlepass::default();

    let cost_function = |_: &Operator| -> u64 { 1 };
    let metering = Arc::new(Metering::new(metering_limit, cost_function));
    config.push_middleware(metering);

    let engine = EngineBuilder::new(config);

    let base = BaseTunables::for_target(&Target::default());
    let tunables = LimitingTunables::new(base, Pages(memory_limit_page));
    let store = Store::new(engine);
    let store_engine = store.engine();
    store_engine.to_owned().set_tunables(tunables);

    let module = Module::new(&store, code).map_err(|original| Error::CompileError {
        msg: format!("{original}"),
    })?;

    Ok((module, store))
}
