use minicbor::{Encode, Decode};

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
#[cbor(array)]
pub enum TestResponse {
    #[b(0)]
    I32 {
        #[b(0)]
        value: i32,
    },
}

#[derive(Clone, Debug, Decode, Encode)]
#[cbor(array)]
pub enum TestError {
    #[b(0)]
    DivByZero,
}
