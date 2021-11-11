pub mod contract;
pub mod error;
pub mod provider_api;

mod executor;
mod memory;
mod page;
mod state;
mod wasmer;

pub type Address = [u8; 20];
pub type Hash32 = [u8; 32];

pub const ONE_KB: u64 = 1024;
pub const ONE_MB: u64 = ONE_KB * ONE_KB;
pub const ONE_GB: u64 = ONE_KB * ONE_KB * ONE_KB;

#[allow(dead_code)]
pub fn address_from_hex(s: &str) -> Address {
    let mut addr: Address = [0u8; 20];
    let src = &hex::decode(s).unwrap();
    addr.copy_from_slice(src);
    addr
}
