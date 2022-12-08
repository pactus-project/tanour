use hex_literal::hex;
use tanour::{contract::Contract, provider_mock::ProviderMock};
use test_contract::message::{Error, InstantiateMsg, ProcMsg, QueryMsg, QueryRsp};

#[test]
fn test_call_process() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    //TODO: define const for Mega and Kilo
    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000, 100000).unwrap();

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let msg = ProcMsg::Null;
    let res: Result<(), Error> = contract.call_process(&msg).unwrap();
    assert!(res.is_ok());
    assert_eq!(contract.consumed_points().unwrap(), 10016); // TODO: This is not accurate. by changing the wasmer version it will change. We should get rid of it
}

#[test]
fn test_read_write_storage() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000, 100000).unwrap();

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let msg = "hello world!".to_string();
    let _: Result<(), Error> = contract
        .call_process(&ProcMsg::SetMessage { msg: msg })
        .unwrap();
    assert_eq!(contract.consumed_points().unwrap(), 12955);

    let res: Result<QueryRsp, Error> = contract.call_query(&QueryMsg::GetMessage).unwrap();
    assert_eq!(res.unwrap(), QueryRsp::String("hello world!".to_string()),);
    assert_eq!(contract.consumed_points().unwrap(), 19008);
    assert!(!contract.exhausted().unwrap());
}

#[test]
fn test_hash_blake2b() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();

    let provider = ProviderMock::new(1024 * 1024);
    let mut contract = Contract::new(provider, &code, 1000000, 100000).unwrap();

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let data = "zarb".as_bytes().to_vec();
   let res: Result<QueryRsp, Error> = contract.call_query(&QueryMsg::Hasher { data }).unwrap();
    assert_eq!(
        res.unwrap(),
        QueryRsp::Data(
            hex!("12b38977f2d67f06f0c0cd54aaf7324cf4fee184398ea33d295e8d1543c2ee1a").to_vec()
        ),
    );
    assert_eq!(contract.consumed_points().unwrap(), 29770);
    assert!(!contract.exhausted().unwrap());
}
