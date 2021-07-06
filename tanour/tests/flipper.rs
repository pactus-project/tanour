mod common;
mod provider;

use crate::provider::Blockchain;
use log::info;
use tanour::execute::execute;
use tanour::action::Action;
use tanour::types::Hash32;

#[test]
fn test_flipper_contract() {
    common::start_logger();

    let mut bc = Blockchain::new();

    let alice = common::address_from_hex("deadbeef00000000000000000000000000000000");
    let bob = common::address_from_hex("deafdad000000000000000000000000000000000");
    bc.add_account(alice);

    let code = include_bytes!("./contracts/flipper/flipper.wasm");
    let params1 = vec![1,0,0,0];
    let tx1 =
        Transaction::make_create(alice, 0, 1000000000, 0, code.to_vec(), params1, Hash32::zero());

    let ret1 = execute(&mut bc, &tx1);
    info!("ret2: {:?}", ret1);
    assert!(ret1.is_ok());

    let contract = ret1.unwrap().contract;

    // // transfer: alice to bob: 0xfae3a09d
    // let mut params2 = vec![0xa9, 0x05, 0x9c, 0xbb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // params2.append(&mut bob.clone().as_bytes_mut().to_vec());
    // params2.append(&mut vec![50]);

    // let tx2 = Transaction::make_call(
    //     alice,
    //     contract,
    //     0,
    //     1000000,
    //     0,
    //     params2,
    // );

    // let ret2 = execute(&mut bc, &tx2);
    // info!("ret2: {:?}", ret2);
    // assert!(ret2.is_ok());

    // // total_supply: 0xdcb736b5
    // let params3 = vec![0xdc, 0xb7, 0x36, 0xb5];
    // let tx3 = Transaction::make_call(alice, contract, 50, 1000000, 0, params3);
    // let ret3 = execute(&mut bc, &tx3);
    // info!("ret3: {:?}", ret3);
    // assert!(ret3.is_ok());
}
