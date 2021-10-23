use super::*;
use crate::state::MockStateTrait;
use crate::wasmer::compile::compile;
use wasmer::ImportObject;
use wasmer::Pages;

const ONE_KB: u64 = 1000;
const ONE_MB: u64 = ONE_KB * 1000;

fn make_env(
    wat: &str,
    mem_limit: u64,
    resolver: &ImportObject,
    state: MockStateTrait,
) -> (Env, Box<WasmerInstance>) {
    let code = wat::parse_str(wat).unwrap();
    let module = compile(&code, mem_limit).unwrap();
    let instance = Box::new(wasmer::Instance::new(&module, &resolver).unwrap());
    let instance_ptr = NonNull::from(instance.as_ref());
    let env = Env::new(Arc::new(Mutex::new(state)));
    env.set_instance(Some(instance_ptr));

    (env, instance)
}

#[test]
fn call_no_instance() {
    let state_mock = MockStateTrait::new();
    let env = Env::new(Arc::new(Mutex::new(state_mock)));

    let res = env.call_function("allocate", &[]);
    match res.unwrap_err() {
        Error::InstantiationError { msg } => assert!(msg.contains("Wasmer instance")),
        err => panic!("Unexpected error: {:?}", err),
    }
}

#[test]
fn test_exported_memory() {
    let wat = r#"
(module
    (memory 4)
    (export "memory" (memory 0))
)"#;
    let (env, _instance) = make_env(wat, ONE_MB, &ImportObject::new(), MockStateTrait::new());
    let mem = env.memory().unwrap();

    assert_eq!(mem.ty().minimum, Pages(4));
    assert_eq!(mem.ty().maximum, Some(Pages(15)));
}

#[test]
fn test_call_no_params() {
    let wat = r#"
(module
    (type $t0 (func))
    (func $nope (type $t0))
    (export "nope" (func $nope))
)"#;

    let (env, _instance) = make_env(wat, 0, &ImportObject::new(), MockStateTrait::new());
    let res = env.call_function("nope", &[]);
    assert!(res.is_ok());
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

    let (env, _instance) = make_env(wat, 0, &ImportObject::new(), MockStateTrait::new());

    let res = env
        .call_function("add", &[Val::I32(1), Val::I32(2)])
        .unwrap();
    assert_eq!(res.to_vec(), vec![Val::I32(3)]);
}
