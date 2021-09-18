use test_contract::message::TestMsg;
use wasmer::{Memory, Pages};

use crate::{
    contract::Contract, executor::Value, provider_api::provider_mock::ProviderMock, utils,
};

#[test]
fn test_exported_memory() {
    let wat = r#"
(module
    (memory 4)
    (export "memory" (memory 0))
)"#;

    let code = wat::parse_str(wat).unwrap();
    let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

    let provider = ProviderMock::new(1024 * 1024);
    let contract = Contract::new(provider, address, &code, 1000000).unwrap();
    let instance_memory: Memory = contract
        .instance
        .exports
        .iter()
        .memories()
        .map(|pair| pair.1.clone())
        .next()
        .unwrap();
    assert_eq!(instance_memory.ty().minimum, Pages(4));
    assert_eq!(instance_memory.ty().maximum, Some(Pages(15)));
}

#[test]
fn test_call_no_params() {
    let wat = r#"
(module
    (type $t0 (func))
    (func $nope (type $t0))
    (export "nope" (func $nope))
)"#;

    let code = wat::parse_str(wat).unwrap();
    let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

    let provider = ProviderMock::new(1024 * 1024);
    let contract = Contract::new(provider, address, &code, 1000000).unwrap();
    let res = contract.call_function("nope", &[]).unwrap();
    assert!(res.is_none());
}

#[test]
fn test_call_with_params() {
    let wat = r#"
(module
    (type $t0 (func))
    (func $add (param $param0 i32) (param $param1 i32) (result i32)
        (i32.add
            (local.get $param0)
            (local.get $param1)
        )
    )
    (export "add" (func $add))
)"#;

    let code = wat::parse_str(wat).unwrap();
    let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

    let provider = ProviderMock::new(1024 * 1024);
    let contract = Contract::new(provider, address, &code, 1000000).unwrap();
    let res = contract
        .call_function("add", &[Value::I32(1), Value::I32(2)])
        .unwrap()
        .unwrap();
    assert_eq!(res, Value::I32(3));
}

#[test]
fn test_call_process_msg() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

    let provider = ProviderMock::new(1024 * 1024);
    let contract = Contract::new(provider, address, &code, 1000000).unwrap();

    let msg = TestMsg::Mul { a: 2, b: 2 };
    let mut arg = [0u8; 128];
    minicbor::encode(&msg, arg.as_mut()).unwrap();
    let res = contract.call_process_msg_function(&arg).unwrap();
}
