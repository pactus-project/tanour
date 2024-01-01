use hex_literal::hex;
use rand::Rng;
use tanour::{
    blockchain_api::MockBlockchainAPI,
    contract::{Contract, Params},
};
use test_contract::message::{Error, InstantiateMsg, ProcMsg, QueryMsg, QueryRsp};

fn make_test_contract(wat: &[u8], memory_limit_page: u32, metering_limit: u64) -> Contract {
    let code = wat::parse_bytes(wat).unwrap().to_vec();
    let address = rand::thread_rng().gen::<[u8; 21]>();
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

    let arg = InstantiateMsg {};
    let data = minicbor::to_vec(arg).unwrap();
    contract.call_instantiate(&data).unwrap();

    let arg = ProcMsg::Null;
    let encoded_arg = minicbor::to_vec(arg).unwrap();
    contract.call_process(&encoded_arg).unwrap();
    assert_eq!(contract.consumed_points().unwrap(), 9526);
}

#[test]
fn test_read_write_storage() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let mut contract = make_test_contract(wat, 16, 100000);

    let arg = InstantiateMsg {};
    let encoded_arg = minicbor::to_vec(arg).unwrap();
    let encoded_res = contract.call_instantiate(&encoded_arg).unwrap();
    let res = minicbor::decode::<Result<(), Error>>(&encoded_res).unwrap();
    assert!(res.is_ok());

    let arg = ProcMsg::SetMessage {
        msg: "hello world!".to_string(),
    };
    let encoded_arg = minicbor::to_vec(arg).unwrap();
    let encoded_res = contract.call_process(&encoded_arg).unwrap();
    let res = minicbor::decode::<Result<(), Error>>(&encoded_res).unwrap();
    assert!(res.is_ok());
    assert_eq!(contract.consumed_points().unwrap(), 12350);

    let encoded_arg = QueryMsg::GetMessage;
    let data = minicbor::to_vec(encoded_arg).unwrap();
    let encoded_res = contract.call_query(&data).unwrap();
    let res = minicbor::decode::<Result<QueryRsp, Error>>(&encoded_res).unwrap();
    assert_eq!(res.unwrap(), QueryRsp::String("hello world!".to_string()),);
    assert_eq!(contract.consumed_points().unwrap(), 18119);
    assert!(!contract.exhausted().unwrap());
}

#[test]
fn test_hash_blake2b() {
    let wat = include_bytes!("../../test-contract/wasm/test_contract.wasm");
    let mut contract = make_test_contract(wat, 16, 100000);

    let arg = InstantiateMsg {};
    let encoded_arg = minicbor::to_vec(arg).unwrap();
    let encoded_res = contract.call_instantiate(&encoded_arg).unwrap();
    let res = minicbor::decode::<Result<(), Error>>(&encoded_res).unwrap();
    assert!(res.is_ok());

    let arg = QueryMsg::Hasher {
        data: "zarb".as_bytes().to_vec(),
    };
    let encoded_arg = minicbor::to_vec(arg).unwrap();
    let encoded_res = contract.call_query(&encoded_arg).unwrap();
    let res = minicbor::decode::<Result<QueryRsp, Error>>(&encoded_res).unwrap();
    assert_eq!(
        res.unwrap(),
        QueryRsp::Data(
            hex!("12b38977f2d67f06f0c0cd54aaf7324cf4fee184398ea33d295e8d1543c2ee1a").to_vec()
        ),
    );
    assert_eq!(contract.consumed_points().unwrap(), 28598);
    assert!(!contract.exhausted().unwrap());
}
