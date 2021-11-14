use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct InstantiateMsg {}

#[derive(Clone, Debug, Encode, Decode)]
pub enum ProcMsg {
    #[n(0)]
    Null,
    #[n(1)]
    WriteData {
        #[n(0)]
        offset: u32,
        #[n(1)]
        data: Vec<u8>,
    },
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryMsg {
    #[n(0)]
    ReadData {
        #[n(0)]
        offset: u32,
        #[n(1)]
        length: u32,
    },
    #[n(1)]
    Hash {
        #[n(0)]
        data: Vec<u8>,
    },
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub enum QueryRsp {
    #[b(0)]
    Buffer(#[n(0)] Vec<u8>),
}

#[derive(Clone, Debug, Decode, Encode)]
pub enum TestError {
    #[b(0)]
    KelkError,
    #[b(1)]
    DivByZero,
}
