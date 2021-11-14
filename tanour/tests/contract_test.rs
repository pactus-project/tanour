use hex_literal::hex;
use tanour::{contract::Contract, provider_mock::ProviderMock};
use test_contract::message::{ProcMsg, QueryMsg, QueryRsp, TestError};

#[test]
fn test_call_process_msg() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    //TODO: define const for Mega and Kilo
    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000, 100000).unwrap();

    let msg = ProcMsg::Null;
    let res: Result<(), TestError> = contract.call_process_msg(&msg).unwrap();
    assert!(res.is_ok());
    assert_eq!(contract.consumed_points().unwrap(), 2871);
}

#[test]
fn test_read_write_storage() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000, 100000).unwrap();

    let msg = "hello world!".as_bytes();
    let _: Result<(), TestError> = contract
        .call_process_msg(&ProcMsg::WriteData {
            offset: 0,
            data: msg.to_vec(),
        })
        .unwrap();
    assert_eq!(contract.consumed_points().unwrap(), 6596);

    let res: Result<QueryRsp, TestError> = contract
        .call_query(&QueryMsg::ReadData {
            offset: 6,
            length: 5,
        })
        .unwrap();
    assert_eq!(res.unwrap(), QueryRsp::Buffer("world".as_bytes().to_vec()),);
    assert_eq!(contract.consumed_points().unwrap(), 10859);
    assert!(!contract.exhausted().unwrap());
}

#[test]
fn test_hash_blake2b() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000, 100000).unwrap();
    let data = "zarb".as_bytes().to_vec();
    let res: Result<QueryRsp, TestError> = contract.call_query(&QueryMsg::Hash { data }).unwrap();

    assert_eq!(
        res.unwrap(),
        QueryRsp::Buffer(
            hex!("12b38977f2d67f06f0c0cd54aaf7324cf4fee184398ea33d295e8d1543c2ee1a").to_vec()
        ),
    );
    assert_eq!(contract.consumed_points().unwrap(), 32364);
    assert!(!contract.exhausted().unwrap());
}
