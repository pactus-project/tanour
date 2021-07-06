#[cfg(test)]
mod tests {

    use crate::{
        action, execute, provider::provider_test::ProviderMock, provider::Provider, utils,
    };

    #[test]
    fn test_instantiate() {
        let sum_wat = "(module
            (type $t0 (func))
            (type $t1 (func (result i32)))
            (import \"env\" \"memory\" (memory $env.memory 1))
            (func $deploy (type $t0))
            (func $call (type $t1) (result i32)
              i32.const 3)
            (global $g0 (mut i32) (i32.const 65536))
            (global $__data_end i32 (i32.const 65536))
            (global $__heap_base i32 (i32.const 65536))
            (export \"deploy\" (func $deploy))
            (export \"call\" (func $call))
            (export \"__data_end\" (global 1))
            (export \"__heap_base\" (global 2)))";

        let code = wat::parse_str(sum_wat).unwrap();
        let args = hex::decode("00").unwrap();
        let caller = utils::address_from_hex("deadbeef00000000000000000000000000000000");
        let code_address = utils::address_from_hex("deadbeef00000000000000000000000000000000");

        let action = action::Action {
            caller,
            code_address,
            gas_limit: 1,
            memory_limit: 100000,
            value: 0,
            code,
            args,
        };
        let mut p = ProviderMock {};
        let res = execute::execute(&mut p, &action);
        print!("err {:?}", res);
        assert!(res.is_ok());
    }
}
