use minicbor::{Decode, Encode};


#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
#[cbor(array)]
pub enum TestResponse {
    #[b(0)]
    Null,

    #[b(1)]
    I32 {
        #[b(0)]
        value: i32,
    },

    #[b(2)]
    Buffer(#[n(0)] Vec<u8>),
}

#[derive(Clone, Debug, Decode, Encode)]
#[cbor(array)]
pub enum TestError {
    #[b(0)]
    DivByZero,

    #[b(100)]
    KelkError,
}
