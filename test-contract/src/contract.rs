use crate::message::{InstantiateMsg, ProcMsg, QueryMsg, QueryRsp, TestError};
use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use kelk::context::Context;
use kelk::kelk_derive;

fn null(_ctx: Context) -> Result<(), TestError> {
    Ok(())
}

fn write_buffer(ctx: Context, offset: u32, data: &[u8]) -> Result<(), TestError> {
    ctx.storage
        .write(offset, data)
        .map_err(|_| TestError::KelkError)
}

fn read_buffer(ctx: Context, offset: u32, length: u32) -> Result<Vec<u8>, TestError> {
    ctx.storage
        .read(offset, length)
        .map_err(|_| TestError::KelkError)
}

fn get_hash(_ctx: Context, data: Vec<u8>) -> Result<Vec<u8>, TestError> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(data);
    let res = hasher.finalize_boxed().to_vec();

    Ok(res)
}


#[kelk_derive(instantiate)]
pub fn instantiate(_ctx: Context, _msg: InstantiateMsg) -> Result<(), TestError> {
    Ok(())
}

#[kelk_derive(process)]
pub fn process(ctx: Context, msg: ProcMsg) -> Result<(), TestError> {
    match msg {
        ProcMsg::Null => null(ctx),
        ProcMsg::WriteData { offset, data } => write_buffer(ctx, offset, &data),
    }
}

#[kelk_derive(query)]
pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, TestError> {
    match msg {
        QueryMsg::ReadData { offset, length } => {
            Ok(QueryRsp::Buffer(read_buffer(ctx, offset, length)?))
        }
        QueryMsg::Hash { data } => Ok(QueryRsp::Buffer(get_hash(ctx, data)?)),
    }
}
