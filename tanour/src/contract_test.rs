#[cfg(test)]
mod tests {
    use crate::{
        action::{Action, CallMethod},
        contract::Contract,
        provider::provider_test::ProviderMock,
        utils,
    };

    #[test]
    fn test_instantiate() {
        let sum_wat = r#"(module
            (type $t0 (func))
            (type $t1 (func (result i32)))
            (func $deploy (type $t0))
            (func $execute (type $t1) (result i32)
              i32.const 3)
            (export "deploy" (func $deploy))
            (export "execute" (func $execute)))"#;

        let code = wat::parse_str(sum_wat).unwrap();
        let args = hex::decode("00").unwrap();
        let caller = utils::address_from_hex("deadbeef00000000000000000000000000000000");
        let code_address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

        let action = Action::new(caller, 1000000, 1, CallMethod::Deploy, args.clone());
        let mut p = ProviderMock {};
        let contract = Contract::instantiate(code_address, &code, 1000000).unwrap();
        let res = contract.call_function("execute", &[&args]).unwrap();
        print!("{:?}", res);
    }
}
