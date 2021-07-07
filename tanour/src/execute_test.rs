#[cfg(test)]
mod tests {
    use wasmer::wat2wasm;
    use crate::{
        action, execute, provider::provider_test::ProviderMock, provider::Provider, utils,
    };

    #[test]
    fn test_instantiate() {
        let sum_wat = r#"(module (memory 7) (export "memory" (memory 0)))"#;

        let code = wat::parse_str(sum_wat).unwrap();
        let args = hex::decode("00").unwrap();
        let caller = utils::address_from_hex("deadbeef00000000000000000000000000000000");
        let code_address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

        let action = action::Action {
            caller,
            code_address,
            gas_limit: 1,
            memory_limit: 1000000,
            value: 0,
            code,
            args,
        };
        let mut p = ProviderMock {};
        let res = execute::execute(&mut p, &action);
        print!("err {:?}", res);
        //assert!(res.is_ok());
    }
}
