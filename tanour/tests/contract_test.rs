use hex_literal::hex;
use tanour::{
    blockchain_api::MockBlockchainAPI,
    contract::{Contract, Params},
};
use test_contract::message::{Error, InstantiateMsg, ProcMsg, QueryMsg, QueryRsp};

fn make_test_contract(wat: &[u8], memory_limit_page: u32, metering_limit: u64) -> Contract {
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = rand::random();
    let params = Params {
        memory_limit_page,
        metering_limit,
    };

    let mut api = Box::new(MockBlockchainAPI::new());
    api.expect_page_size().returning(|| Ok(256));
    api.expect_read_page().returning(|_| Ok(vec![0; 256]));

    Contract::new(api, &address, &code, params).unwrap()
}

#[test]
fn test_call_process() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let mut contract = make_test_contract(wat, 16, 10000);

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let msg = ProcMsg::Null;
    let res: Result<(), Error> = contract.call_process(&msg).unwrap();
    assert!(res.is_ok());
    assert_eq!(contract.consumed_points().unwrap(), 9526);
}

#[test]
fn test_read_write_storage() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let mut contract = make_test_contract(wat, 16, 100000);
    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let msg = "hello world!".to_string();
    let _: Result<(), Error> = contract.call_process(&ProcMsg::SetMessage { msg }).unwrap();
    assert_eq!(contract.consumed_points().unwrap(), 12350);

    let res: Result<QueryRsp, Error> = contract.call_query(&QueryMsg::GetMessage).unwrap();
    assert_eq!(res.unwrap(), QueryRsp::String("hello world!".to_string()),);
    assert_eq!(contract.consumed_points().unwrap(), 18119);
    assert!(!contract.exhausted().unwrap());
}

#[test]
fn test_hash_blake2b() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let mut contract = make_test_contract(wat, 16, 100000);

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let data = "zarb".as_bytes().to_vec();
    let res: Result<QueryRsp, Error> = contract.call_query(&QueryMsg::Hasher { data }).unwrap();
    assert_eq!(
        res.unwrap(),
        QueryRsp::Data(
            hex!("12b38977f2d67f06f0c0cd54aaf7324cf4fee184398ea33d295e8d1543c2ee1a").to_vec()
        ),
    );
    assert_eq!(contract.consumed_points().unwrap(), 28598);
    assert!(!contract.exhausted().unwrap());
}
