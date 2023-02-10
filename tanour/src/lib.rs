pub mod blockchain_api;
pub mod contract;
pub mod error;

mod executor;
mod memory;
mod page;
mod provider;
mod storage_file;
mod wasmer;
#[cfg(test)]
use rand::prelude::*;

pub const ADDRESS_SIZE: usize = 21;

pub type Address = [u8; ADDRESS_SIZE];

#[cfg(test)]
pub fn address_from_hex(s: &str) -> Address {
    let mut addr: Address = [0u8; ADDRESS_SIZE];
    let src = &hex::decode(s).unwrap();
    addr.copy_from_slice(src);
    addr
}

pub fn address_to_hex(addr: &Address) -> String {
    hex::encode(addr)
}

#[cfg(test)]
pub fn random_address() -> Address {
    let mut rng = rand::thread_rng();
    rng.gen()
}
