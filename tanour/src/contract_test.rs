#[cfg(test)]
mod tests {
    use wasmer::{Memory, Pages, Val};

    use crate::{contract::Contract, provider_api::provider_mock::ProviderMock, utils};

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
        assert_eq!(res.to_vec(), Vec::new());
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
        let res = contract.call_function("add", &[Val::I32(1), Val::I32(2)]).unwrap();
        assert_eq!(res.to_vec(), vec![Val::I32(3)]);
    }
}
