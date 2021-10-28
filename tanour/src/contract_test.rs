use test_contract::{
    message::TestMsg,
    result::{TestError, TestResponse},
};

use crate::{contract::Contract, provider_api::provider_mock::ProviderMock};

#[test]
fn test_call_process_msg() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    //TODO: define const for Mega and Kilo
    let provider = ProviderMock::new(1024);
    let mut contract = Contract::new(provider, &code, 1000000).unwrap();

    let msg = TestMsg::Mul { a: 2, b: 2 };
    let res: Result<TestResponse, TestError> = contract.call_process_msg(&msg).unwrap();
    assert_eq!(res.unwrap(), TestResponse::I32 { value: 4 });

    let msg = TestMsg::Div { a: 2, b: 0 };
    let res: Result<TestResponse, TestError> = contract.call_process_msg(&msg).unwrap();
    assert!(res.is_err());
}

#[test]
fn test_read_write_storage() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000).unwrap();

    let msg = "hello world!".as_bytes();
    let _: Result<TestResponse, TestError> = contract
        .call_process_msg(&TestMsg::WriteData {
            offset: 0,
            data: msg.to_vec(),
        })
        .unwrap();

    let res: Result<TestResponse, TestError> = contract
        .call_process_msg(&TestMsg::ReadData {
            offset: 6,
            length: 5,
        })
        .unwrap();
    assert_eq!(
        res.unwrap(),
        TestResponse::Buffer("world".as_bytes().to_vec()),
    );
}
