pub mod blockchain_api;
pub mod contract;
pub mod error;

mod executor;
mod memory;
mod page;
mod provider;
mod storage_file;
mod wasmer;

pub const ADDRESS_SIZE: usize = 21;

pub type Address = [u8; ADDRESS_SIZE];
pub type Hash32 = [u8; 32];

#[allow(dead_code)]
pub fn address_from_hex(s: &str) -> Address {
    let mut addr: Address = [0u8; ADDRESS_SIZE];
    let src = &hex::decode(s).unwrap();
    addr.copy_from_slice(src);
    addr
}

pub fn address_to_hex(addr: &Address) -> String {
    hex::encode(addr)
}
