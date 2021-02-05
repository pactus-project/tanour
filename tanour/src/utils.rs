use crate::types::{Address, Hash32};
use keccak_hash::write_keccak;
use primitive_types::H256;

pub fn keccak<T: AsRef<[u8]>>(s: T) -> Hash32 {
    let mut result = [0u8; 32];
    write_keccak(s, &mut result);
    H256(result)
}

pub fn contract_address(sender: &Address, code: &[u8], salt: &Hash32) -> Address {
    let code_hash = keccak(code);
    let mut buffer = [0u8; 1 + 20 + 32 + 32];
    buffer[0] = 0xff;
    &mut buffer[1..(1 + 20)].copy_from_slice(&sender[..]);
    &mut buffer[(1 + 20)..(1 + 20 + 32)].copy_from_slice(&salt[..]);
    &mut buffer[(1 + 20 + 32)..].copy_from_slice(&code_hash[..]);
    From::from(keccak(&buffer[..]))
}

#[cfg(test)]
mod test {
    use super::*;
    use hex::decode;

    #[test]
    fn test_contract_address() {
        // Test cases from Etheruem project
        struct TestCase<'a> {
            origin: &'a str,
            salt: &'a str,
            code: &'a str,
            expected: &'a str,
        }

        let test_cases = vec![
            TestCase{
                origin:   "0000000000000000000000000000000000000000",
                salt:     "0000000000000000000000000000000000000000000000000000000000000000",
                code:     "00",
                expected: "4d1a2e2bb4f88f0250f26ffff098b0b30b26bf38",
            },
            TestCase{
                origin:   "deadbeef00000000000000000000000000000000",
                salt:     "0000000000000000000000000000000000000000000000000000000000000000",
                code:     "00",
                expected: "B928f69Bb1D91Cd65274e3c79d8986362984fDA3",
            },
            TestCase{
                origin:   "deadbeef00000000000000000000000000000000",
                salt:     "000000000000000000000000feed000000000000000000000000000000000000",
                code:     "00",
                expected: "D04116cDd17beBE565EB2422F2497E06cC1C9833",
            },
            TestCase{
                origin:   "0000000000000000000000000000000000000000",
                salt:     "0000000000000000000000000000000000000000000000000000000000000000",
                code:     "deadbeef",
                expected: "70f2b2914A2a4b783FaEFb75f459A580616Fcb5e",
            },
            TestCase{
                origin:   "00000000000000000000000000000000deadbeef",
                salt:     "00000000000000000000000000000000000000000000000000000000cafebabe",
                code:     "deadbeef",
                expected: "60f3f640a8508fC6a86d45DF051962668E1e8AC7",
            },
            TestCase{
                origin:   "00000000000000000000000000000000deadbeef",
                salt:     "00000000000000000000000000000000000000000000000000000000cafebabe",
                code:     "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                expected: "1d8bfDC5D46DC4f61D6b6115972536eBE6A8854C",
            },
            TestCase{
                origin:   "0000000000000000000000000000000000000000",
                salt:     "0000000000000000000000000000000000000000000000000000000000000000",
                code:     "",
                expected: "E33C0C7F7df4809055C3ebA6c09CFe4BaF1BD9e0",
            },
        ];

        for t in test_cases {
            let origin = decode(t.origin).unwrap();
            let salt = decode(t.salt).unwrap();
            let code = decode(t.code).unwrap();
            let expected = decode(t.expected).unwrap();

            let addr = Address::from_slice(&origin);
            let salt = Hash32::from_slice(&salt);
            let expected = Address::from_slice(&expected);
            let contract = contract_address(&addr, &code, &salt);

            assert_eq!(expected, contract);
        }
    }
}
