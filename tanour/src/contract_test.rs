use test_contract::{
    message::TestMsg,
    result::{TestError, TestResponse},
};

use crate::{contract::Contract, provider_api::provider_mock::ProviderMock};

#[test]
fn test_call_process_msg() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000).unwrap();

    let msg = TestMsg::Mul { a: 2, b: 2 };
    let res: Result<TestResponse, TestError> = contract.call_process_msg(&msg).unwrap();
    assert_eq!(res.unwrap(), TestResponse::I32 { value: 4 });
}
