use crate::tanour_capnp;

use log::debug;

use tanour::{blockchain_api::BlockchainAPI, Address};

unsafe impl Send for tanour_capnp::provider::Client {}

pub struct BlockchainAdaptor {
    client: tanour_capnp::provider::Client,
}

impl BlockchainAdaptor {
    pub fn new(client: tanour_capnp::provider::Client) -> Self {
        BlockchainAdaptor { client }
    }
}

impl BlockchainAPI for BlockchainAdaptor {
    fn exist(&self, address: &Address) -> Result<bool, tanour::error::Error> {
        let mut request = self.client.exists_request();
        request.get().set_address(address);

        let handle = async move {
            debug!("Try ot call `exists` method in client");
            let result = request.send().promise.await.unwrap(); //TODO: no unwrap
            result.get().unwrap().get_exist() //TODO: no unwrap
        };

        Ok(futures::executor::block_on(handle))
    }

    fn current_block_number(&self) -> u32 {
        todo!()
    }

    // fn account(&self, address: &Address) -> Result<StateAccount, tanour::error::Error> {
    //     let mut request = self.client.account_request();
    //     {
    //         request.get().set_address(address.as_bytes());
    //     }
    //     let handle = async move {
    //         debug!("Try ot call `account` method in client");
    //         let result = request.send().promise.await?;
    //         let account = result.get()?.get_account()?;

    //         Ok(StateAccount {
    //             nonce: U256::from_little_endian(account.get_nonce()?),
    //             balance: U256::from_little_endian(account.get_balance()?),
    //             code: account.get_code()?.to_vec(),
    //         })
    //     };

    //     futures::executor::block_on(handle).map_err(|e: Error| e.into())
    // }

    // fn create_contract(
    //     &mut self,
    //     address: &Address,
    //     code: &Vec<u8>,
    // ) -> Result<(), tanour::error::Error> {
    //     let mut request = self.client.create_contract_request();
    //     {
    //         request.get().set_address(address.as_bytes());
    //         request.get().set_code(code);
    //     }
    //     let handle = async move {
    //         debug!("Try ot call `create_contract` method in client");
    //         request.send().promise.await?;

    //         Ok(())
    //     };

    //     futures::executor::block_on(handle).map_err(|e: Error| e.into())
    // }

    // fn update_account(
    //     &mut self,
    //     address: &Address,
    //     balance: &U256,
    //     nonce: &U256,
    // ) -> Result<(), tanour::error::Error> {
    //     let mut request = self.client.update_account_request();
    //     {
    //         let mut tmp = Vec::new();
    //         tmp.resize(32, 0);

    //         request.get().set_address(address.as_bytes());

    //         balance.to_little_endian(&mut tmp);
    //         request.get().set_balance(&tmp);

    //         nonce.to_little_endian(&mut tmp);
    //         request.get().set_nonce(&tmp);
    //     }
    //     let handle = async move {
    //         debug!("Try ot call `update_account` method in client");
    //         request.send().promise.await?;

    //         Ok(())
    //     };

    //     futures::executor::block_on(handle).map_err(|e: Error| e.into())
    // }

    // fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, tanour::error::Error> {
    //     let mut request = self.client.storage_at_request();
    //     {
    //         request.get().set_address(address.as_bytes());
    //         request.get().set_key(key.as_bytes());
    //     }
    //     let handle = async move {
    //         debug!("Try ot call `storage_at` method in client");
    //         let result = request.send().promise.await?;
    //         let storage = result.get()?.get_storage()?;

    //         Ok(H256::from_slice(storage))
    //     };

    //     futures::executor::block_on(handle).map_err(|e: Error| e.into())
    // }

    // fn set_storage(
    //     &mut self,
    //     address: &Address,
    //     key: &H256,
    //     value: &H256,
    // ) -> Result<(), tanour::error::Error> {
    //     let mut request = self.client.set_storage_request();
    //     {
    //         request.get().set_address(address.as_bytes());
    //         request.get().set_key(key.as_bytes());
    //         request.get().set_value(value.as_bytes());
    //     }
    //     let handle = async move {
    //         debug!("Try ot call `set_storage` method in client");
    //         request.send().promise.await?;

    //         Ok(())
    //     };

    //     futures::executor::block_on(handle).map_err(|e: Error| e.into())
    // }

    // fn block_hash(&self, _num: u64) -> Result<H256, tanour::error::Error> {
    //     Ok(H256::zero())
    // }

    // fn timestamp(&self) -> u64 {
    //     0
    // }

    // fn block_number(&self) -> u64 {
    //     0
    // }

    // fn block_author(&self) -> Result<Address, tanour::error::Error> {
    //     Err(tanour::error::Error::NotSupported)
    // }

    // fn difficulty(&self) -> Result<U256, tanour::error::Error> {
    //     Err(tanour::error::Error::NotSupported)
    // }

    // fn gas_limit(&self) -> Result<U256, tanour::error::Error> {
    //     Ok(U256::zero())
    // }
}
