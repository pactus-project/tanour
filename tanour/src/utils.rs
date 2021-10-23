use crate::types::Address;

pub fn address_from_hex(s: &str) -> Address {
    let mut addr: Address = [0u8; 20];
    let src = &hex::decode(s).unwrap();
    addr.copy_from_slice(src);
    addr
}
