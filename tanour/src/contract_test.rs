use test_contract::message::TestMsg;

use crate::{contract::Contract, provider_api::provider_mock::ProviderMock, utils};


#[test]
fn test_call_process_msg() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

    let provider = ProviderMock::new(1024 * 1024);
    let contract = Contract::new(provider, address, &code, 1000000).unwrap();

    let msg = TestMsg::Mul { a: 2, b: 2 };
    let mut arg = [0u8; 128];
    let res = contract.call_process_msg(&msg).unwrap();
}
