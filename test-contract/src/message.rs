use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub struct InstantiateMsg {}

#[derive(Clone, Debug, Encode, Decode)]
pub enum ProcMsg {
    #[n(0)]
    Null,
    #[n(1)]
    SetMessage {
        #[n(0)]
        msg: String,
    },
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum QueryMsg {
    #[n(0)]
    GetMessage,
    #[n(1)]
    Hasher {
        #[n(0)]
        data: Vec<u8>,
    },
    #[n(2)]
    Divider {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
}

#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq)]
pub enum QueryRsp {
    #[n(0)]
    String(#[n(0)] String),
    #[n(1)]
    Data(#[n(0)] Vec<u8>),
    #[n(2)]
    Int32(#[n(0)] i32),
}

#[derive(Clone, Debug, Decode, Encode)]
pub enum Error {
    #[n(0)]
    StorageError,
    #[n(1)]
    DivByZero,
}

impl From<kelk::storage::error::Error> for Error {
    fn from(_error: kelk::storage::error::Error) -> Self {
        Error::StorageError
    }
}
