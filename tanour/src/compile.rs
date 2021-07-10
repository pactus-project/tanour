use crate::action::Action;
use crate::error::Error;
use crate::memory;
use crate::provider::Provider;
use crate::types::Address;
use crate::utils;
use log::{debug, trace};
#[cfg(feature = "cranelift")]
use wasmer::Cranelift;
#[cfg(not(feature = "cranelift"))]
use wasmer::Singlepass;
use wasmer::{
    wasmparser::Operator, BaseTunables, CompilerConfig, Engine, Pages, Store, Target, Universal,
    WASM_PAGE_SIZE,
};
use wasmer::{Exports, Function, ImportObject, Instance as WasmerInstance, Module, Val};

