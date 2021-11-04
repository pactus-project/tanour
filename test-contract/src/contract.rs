use crate::message::TestMsg;
use crate::result::{TestError, TestResponse};
use kelk::context::ContextMut;

fn mul(a: i32, b: i32) -> Result<i32, TestError> {
    Ok(a * b)
}

fn div(a: i32, b: i32) -> Result<i32, TestError> {
    if b == 0 {
        return Err(TestError::DivByZero);
    }
    Ok(a / b)
}

fn write_buffer(ctx: ContextMut, offset: u32, data: &[u8]) -> Result<(), TestError> {
    ctx.api
        .write_storage(offset, data)
        .map_err(|_| TestError::KelkError)
}

fn read_buffer(ctx: ContextMut, offset: u32, length: u32) -> Result<Vec<u8>, TestError> {
    ctx.api
        .read_storage(offset, length)
        .map_err(|_| TestError::KelkError)
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
pub fn instantiate(_ctx: ContextMut) -> Result<TestResponse, TestError> {
    Ok(TestResponse::I32 { value: 0 })
}

/// The process_msg function is the main function of the *deployed* contract actor
// #[kelk_derive(process_msg)]
pub fn process_msg(ctx: ContextMut, msg: TestMsg) -> Result<TestResponse, TestError> {
    let res = match msg {
        TestMsg::Mul { a, b } => TestResponse::I32 { value: mul(a, b)? },
        TestMsg::Div { a, b } => TestResponse::I32 { value: div(a, b)? },
        TestMsg::WriteData { offset, data } => {
            write_buffer(ctx, offset, &data)?;
            TestResponse::Null
        }
        TestMsg::ReadData { offset, length } => {
            TestResponse::Buffer(read_buffer(ctx, offset, length)?)
        }
    };

    Ok(res)
}
