use crate::{error::Result, Address};
use mockall::{automock, predicate::*};

#[automock]
pub trait BlockchainAPI: Send + 'static {
    fn exist(&self, address: &Address) -> Result<bool>;
    // TODO: maybe better we return a block_info, including hash, time, number and proposer address
    fn current_block_number(&self) -> u32;
}
