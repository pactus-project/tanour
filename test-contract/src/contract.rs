use crate::message::{ProcMsg, QueryMsg, QueryRsp, TestError};
use kelk_env::context::Context;

fn null(_ctx: Context) -> Result<(), TestError> {
    Ok(())
}

fn write_buffer(ctx: Context, offset: u32, data: &[u8]) -> Result<(), TestError> {
    ctx.api
        .swrite(offset, data)
        .map_err(|_| TestError::KelkError)
}

fn read_buffer(ctx: Context, offset: u32, length: u32) -> Result<Vec<u8>, TestError> {
    ctx.api
        .sread(offset, length)
        .map_err(|_| TestError::KelkError)
}

/// The "instantiate" will be executed only once on instantiating the contract actor
#[cfg(target_arch = "wasm32")]
mod __wasm_export_instantiate {
    #[no_mangle]
    extern "C" fn instantiate() -> u32 {
        kelk_env::do_instantiate(&super::instantiate)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: u64) -> u64 {
        kelk_env::do_process_msg(&super::process_msg, msg_ptr)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_query {
    #[no_mangle]
    extern "C" fn query(msg_ptr: u64) -> u64 {
        kelk_env::do_query(&super::query, msg_ptr)
    }
}

// #[kelk_derive(instantiate)]
pub fn instantiate(_ctx: Context) -> Result<(), TestError> {
    Ok(())
}

pub fn process_msg(ctx: Context, msg: ProcMsg) -> Result<(), TestError> {
    match msg {
        ProcMsg::Null => null(ctx),
        ProcMsg::WriteData { offset, data } => write_buffer(ctx, offset, &data),
    }
}

pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, TestError> {
    match msg {
        QueryMsg::ReadData { offset, length } => {
            Ok(QueryRsp::Buffer(read_buffer(ctx, offset, length)?))
        }
    }
}
