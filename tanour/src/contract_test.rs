#[cfg(test)]
mod tests {
    use wasmer::{Memory, Pages};

    use crate::{
        action::{Action, CallMethod},
        contract::Contract,
        provider_api::provider_mock::ProviderMock,
        utils,
    };

    #[test]

    fn test_exported_memory() {
        let sum_wat = r#"(module
            (memory 4)
            (export "memory" (memory 0))
        )"#;

        let code = wat::parse_str(sum_wat).unwrap();
        let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

        let provider = ProviderMock::new(1024 * 1024);
        let contract = Contract::instantiate(provider, address, &code, 1000000).unwrap();
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
    fn test_call() {
        let sum_wat = r#"(module
            (type $t0 (func))
            (func $func (type $t0))
            (export "func" (func $func)))"#;

        let code = wat::parse_str(sum_wat).unwrap();
        let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

        let provider = ProviderMock::new(1024 * 1024);
        let contract = Contract::instantiate(provider, address, &code, 1000000).unwrap();
        let res = contract.call_function("func", &[]);
        assert!(res.is_ok());
    }
}
