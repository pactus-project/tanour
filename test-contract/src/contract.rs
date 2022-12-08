use crate::message::{Error, InstantiateMsg, ProcMsg, QueryMsg, QueryRsp};
use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use kelk::context::Context;
use kelk::kelk_entry;
use kelk::storage::str::StorageString;

fn null(_ctx: Context) -> Result<(), Error> {
    Ok(())
}

fn set_message(ctx: Context, msg: &str) -> Result<(), Error> {
    let offset = ctx.storage.read_stack_at(1)?;
    let mut storage_string = StorageString::load(ctx.storage, offset)?;
    Ok(storage_string.set_string(msg)?)
}

fn get_message(ctx: Context) -> Result<String, Error> {
    let offset = ctx.storage.read_stack_at(1)?;
    let storage_string = StorageString::load(ctx.storage, offset)?;
    Ok(storage_string.get_string()?)
}

fn calc_hash(data: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(data);
    let res = hasher.finalize_boxed().to_vec();

    Ok(res)
}

fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if b == 0 {
        return Err(Error::DivByZero);
    }

    Ok(a / b)
}

#[kelk_entry]
pub fn instantiate(ctx: Context, _msg: InstantiateMsg) -> Result<(), Error> {
    let storage_string = StorageString::create(ctx.storage, 32).unwrap();
    ctx.storage
        .fill_stack_at(1, storage_string.offset())
        .unwrap();
    Ok(())
}

#[kelk_entry]
pub fn process(ctx: Context, msg: ProcMsg) -> Result<(), Error> {
    match &msg {
        ProcMsg::Null => null(ctx),
        ProcMsg::SetMessage { msg } => set_message(ctx, msg),
    }
}

#[kelk_entry]
pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, Error> {
    match msg {
        QueryMsg::GetMessage => Ok(QueryRsp::String(get_message(ctx)?)),
        QueryMsg::Hasher { data } => Ok(QueryRsp::Data(calc_hash(data)?)),
        QueryMsg::Divider { a, b } => Ok(QueryRsp::Int32(divide(a, b)?)),
    }
}
