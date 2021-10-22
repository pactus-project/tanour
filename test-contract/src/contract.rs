use crate::message::TestMsg;
use crate::result::{TestError, TestResponse};
use kelk::{context::ContextMut, kelk_derive};

pub fn mul(a: i32, b: i32) -> Result<i32, TestError> {
    Ok(a * b)
}

pub fn div(a: i32, b: i32) -> Result<i32, TestError> {
    if b == 0 {
        return Err(TestError::DivByZero);
    }
    Ok(a / b)
}

/// The "instantiate" will be executed only once on instantiating the contract actor
#[cfg(target_arch = "wasm32")]
mod __wasm_export_instantiate {
    #[no_mangle]
    extern "C" fn instantiate() -> u32 {
        kelk::do_instantiate(&super::instantiate)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: *const u8, length: u32) -> u64 {
        kelk::do_process_msg(&super::process_msg, msg_ptr, length)
    }
}

// #[kelk_derive(instantiate)]
fn instantiate(context: ContextMut) -> Result<TestResponse, TestError> {
    Ok(TestResponse::I32 { value: 0 })
}

/// The process_msg function is the main function of the *deployed* contract actor
// #[kelk_derive(process_msg)]
fn process_msg(context: ContextMut, msg: TestMsg) -> Result<TestResponse, TestError> {
    let ans = match msg {
        TestMsg::Mul { a, b } => mul(a, b),
        TestMsg::Div { a, b } => div(a, b),
    }?;

    Ok(TestResponse::I32 { value: ans })
}
