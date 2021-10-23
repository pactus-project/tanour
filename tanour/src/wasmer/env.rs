use crate::error::Result;
use crate::{error::Error, state::StateTrait};
use std::sync::RwLock;
use std::{
    borrow::{Borrow, BorrowMut},
    ptr::NonNull,
    sync::{Arc, Mutex},
};
use wasmer::{HostEnvInitError, Instance as WasmerInstance, Memory, Val, WasmerEnv};

pub struct Context {
    /// A non-owning link to the wasmer instance
    instance: Option<NonNull<WasmerInstance>>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new() -> Self {
        Context { instance: None }
    }
}

#[derive(Clone)]
pub struct Env {
    pub state: Arc<Mutex<dyn StateTrait>>,
    context: Arc<RwLock<Context>>,
}

impl Env {
    pub fn new(state: Arc<Mutex<dyn StateTrait>>) -> Self {
        Env {
            state,
            context: Arc::new(RwLock::new(Context::new())),
        }
    }

    /// Creates a back reference from a contact to its partent instance
    pub fn set_instance(&self, instance: Option<NonNull<WasmerInstance>>) {
        self.with_context_mut(|context| {
            context.instance = instance;
        });
    }

    fn with_context_mut<C, R>(&self, callback: C) -> R
    where
        C: FnOnce(&mut Context) -> R,
    {
        let mut guard = self.context.as_ref().write().unwrap();
        let context = guard.borrow_mut();
        callback(context)
    }

    fn with_context<C, R>(&self, callback: C) -> R
    where
        C: FnOnce(&Context) -> R,
    {
        let guard = self.context.as_ref().read().unwrap();
        let context = guard.borrow();
        callback(context)
    }

    pub fn with_instance<C, R>(&self, callback: C) -> Result<R>
    where
        C: FnOnce(&WasmerInstance) -> Result<R>,
    {
        self.with_context(|context| match context.instance {
            Some(instance_ptr) => {
                let instance_ref = unsafe { instance_ptr.as_ref() };
                callback(instance_ref)
            }
            None => Err(Error::InstantiationError {
                msg: "Wasmer instance is not set".to_string(),
            }),
        })
    }

    pub fn memory(&self) -> Result<Memory> {
        self.with_instance(|instance| {
            let first: Option<Memory> = instance
                .exports
                .iter()
                .memories()
                .next()
                .map(|pair| pair.1.clone());

            let memory = first.expect("A contract must have exactly one exported memory.");
            Ok(memory)
        })
    }

    pub fn call_function(&self, name: &str, vals: &[Val]) -> Result<Box<[Val]>> {
        self.with_instance(|instance| {
            let func =
                instance
                    .exports
                    .get_function(name)
                    .map_err(|original| Error::RuntimeError {
                        msg: format!("{}", original),
                    })?;

            func.call(vals).map_err(|original| Error::RuntimeError {
                msg: format!("{}", original),
            })
        })
    }
}

impl WasmerEnv for Env {
    fn init_with_instance(
        &mut self,
        _instance: &wasmer::Instance,
    ) -> std::result::Result<(), HostEnvInitError> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "./env_test.rs"]
mod tests;
