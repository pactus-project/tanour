pub mod contract;
pub mod error;
pub mod provider_api;

mod executor;
mod page;
mod state;
mod wasmer;

pub type Address = [u8; 20];
pub type Hash32 = [u8; 32];

#[allow(dead_code)]
pub fn address_from_hex(s: &str) -> Address {
    let mut addr: Address = [0u8; 20];
    let src = &hex::decode(s).unwrap();
    addr.copy_from_slice(src);
    addr
}
