use super::*;
use wasmer::Pages;

#[test]
fn test_exported_memory() {
    let wat = r#"
(module
    (memory 4)
    (export "memory" (memory 0))
)"#;
    let code = wat::parse_str(wat).unwrap();
    let exec = Executor::new(&code, 1000000).unwrap();
    assert_eq!(exec.memory().unwrap().ty().minimum, Pages(4));
    assert_eq!(exec.memory().unwrap().ty().maximum, Some(Pages(15)));
}

#[test]
fn test_call_no_params() {
    let wat = r#"
(module
    (type $t0 (func))
    (func $nope (type $t0))
    (export "nope" (func $nope))
)"#;

    let exec = Executor::new(wat.as_bytes(), 1024).unwrap();
    let res = exec.call_function("nope", &[]);
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

    let code = wat::parse_str(wat).unwrap();
    let exec = Executor::new(&code, 1024).unwrap();

    let res = exec
        .call_function("add", &[Val::I32(1), Val::I32(2)])
        .unwrap();
    assert_eq!(res.to_vec(), vec![Val::I32(3)]);
}
