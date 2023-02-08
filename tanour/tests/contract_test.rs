use hex_literal::hex;
use tanour::{
    blockchain_api::{MockBlockchainAPI},
    contract::{Contract, Params},
};
use test_contract::message::{Error, InstantiateMsg, ProcMsg, QueryMsg, QueryRsp};

#[test]
fn test_call_process() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = [0; 21]; // TODO!
    let temp_dir = tempfile::tempdir().unwrap();
    let params = Params {
        memory_limit_page: 16,
        metering_limit: 100000,
        storage_path: temp_dir.path().to_str().unwrap().to_string(),
    };
    let blockchain_api = Box::new(MockBlockchainAPI::new());
    let mut contract = Contract::create(blockchain_api, &address, 1, &code, params).unwrap();

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let msg = ProcMsg::Null;
    let res: Result<(), Error> = contract.call_process(&msg).unwrap();
    assert!(res.is_ok());
    assert_eq!(contract.consumed_points().unwrap(), 9522);
}

#[test]
fn test_read_write_storage() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = [0; 21]; // TODO!
    let temp_dir = tempfile::tempdir().unwrap();
    let params = Params {
        memory_limit_page: 16,
        metering_limit: 100000,
        storage_path: temp_dir.path().to_str().unwrap().to_string(),
    };
    let blockchain_api = Box::new(MockBlockchainAPI::new());
    let mut contract = Contract::create(blockchain_api, &address, 1, &code, params).unwrap();

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let msg = "hello world!".to_string();
    let _: Result<(), Error> = contract.call_process(&ProcMsg::SetMessage { msg }).unwrap();
    assert_eq!(contract.consumed_points().unwrap(), 12359);

    let res: Result<QueryRsp, Error> = contract.call_query(&QueryMsg::GetMessage).unwrap();
    assert_eq!(res.unwrap(), QueryRsp::String("hello world!".to_string()),);
    assert_eq!(contract.consumed_points().unwrap(), 18120);
    assert!(!contract.exhausted().unwrap());
}

#[test]
fn test_hash_blake2b() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = [0; 21]; // TODO!
    let temp_dir = tempfile::tempdir().unwrap();
    let params = Params {
        memory_limit_page: 16,
        metering_limit: 100000,
        storage_path: temp_dir.path().to_str().unwrap().to_string(),
    };
    let blockchain_api = Box::new(MockBlockchainAPI::new());
    let mut contract = Contract::create(blockchain_api, &address, 1, &code, params).unwrap();

    let _: Result<(), Error> = contract.call_instantiate(InstantiateMsg {}).unwrap();

    let data = "zarb".as_bytes().to_vec();
    let res: Result<QueryRsp, Error> = contract.call_query(&QueryMsg::Hasher { data }).unwrap();
    assert_eq!(
        res.unwrap(),
        QueryRsp::Data(
            hex!("12b38977f2d67f06f0c0cd54aaf7324cf4fee184398ea33d295e8d1543c2ee1a").to_vec()
        ),
    );
    assert_eq!(contract.consumed_points().unwrap(), 28601);
    assert!(!contract.exhausted().unwrap());
}
