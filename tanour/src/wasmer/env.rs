use wasmer::{HostEnvInitError, Instance as WasmerInstance, WasmerEnv};

#[derive(Clone)]
pub struct Env {}


impl Env {
    pub fn new() -> Self {
        Env{}
    }
}

impl WasmerEnv for Env {
    fn init_with_instance(&mut self, _instance: &WasmerInstance) -> Result<(), HostEnvInitError> {
        Ok(())
    }
}


