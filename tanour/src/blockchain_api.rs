use crate::{error::Result, Address};

use mockall::{automock, predicate::*};

#[automock]
pub trait BlockchainAPI: Send + 'static {
    fn page_size(&self) -> Result<u32>;
    fn read_page(&self, page_no: u32) -> Result<Vec<u8>>;
    fn write_page(&self, page_no: u32, data: &[u8]) -> Result<()>;
    fn exist(&self, address: &Address) -> Result<bool>;
    // TODO: maybe better we return a block_info, including hash, time, number and proposer address
    fn current_block_number(&self) -> u32;
}
