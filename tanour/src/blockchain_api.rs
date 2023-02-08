use crate::{error::Result, Address};
use mockall::{automock, predicate::*};

#[automock]
pub trait BlockchainAPI: Send + 'static {
    fn exist(&self, address: &Address) -> Result<bool>;
}
