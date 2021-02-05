use crate::tanour_capnp;
use log::debug;
use primitive_types::{H256, U256};
use tanour::address::Address;
use tanour::provider::{Provider, StateAccount};
struct Error {
    pub failed: String,
}

impl From<::capnp::Error> for Error {
    fn from(error: ::capnp::Error) -> Self {
        Error {
            failed: error.description,
        }
    }
}
impl From<Error> for tanour::error::Error {
    fn from(error: Error) -> Self {
        tanour::error::Error::Other { msg: error.failed }
    }
}

pub struct ProviderAdaptor {
    client: tanour_capnp::provider::Client,
}

impl ProviderAdaptor {
    pub fn new(client: tanour_capnp::provider::Client) -> Self {
        ProviderAdaptor { client }
    }
}

impl Provider for ProviderAdaptor {
    fn exist(&self, address: &Address) -> bool {
        let mut request = self.client.exist_request();
        {
            request.get().set_address(address.as_bytes());
        }

        let handle = async move {
            debug!("Try ot call `exist` method in client");
            let result = request.send().promise.await?;
            let exist = result.get()?.get_exist();

            Ok(exist)
        };
        let ret: Result<bool, ::capnp::Error> = futures::executor::block_on(handle);
        match ret {
            Ok(exist) => exist,
            Err(_) => false,
        }
    }

    fn account(&self, address: &Address) -> Result<StateAccount, tanour::error::Error> {
        let mut request = self.client.account_request();
        {
            request.get().set_address(address.as_bytes());
        }
        let handle = async move {
            debug!("Try ot call `account` method in client");
            let result = request.send().promise.await?;
            let account = result.get()?.get_account()?;

            Ok(StateAccount {
                nonce: U256::from_little_endian(account.get_nonce()?),
                balance: U256::from_little_endian(account.get_balance()?),
                code: account.get_code()?.to_vec(),
            })
        };

        futures::executor::block_on(handle).map_err(|e: Error| e.into())
    }

    fn create_contract(
        &mut self,
        address: &Address,
        code: &Vec<u8>,
    ) -> Result<(), tanour::error::Error> {
        let mut request = self.client.create_contract_request();
        {
            request.get().set_address(address.as_bytes());
            request.get().set_code(code);
        }
        let handle = async move {
            debug!("Try ot call `create_contract` method in client");
            request.send().promise.await?;

            Ok(())
        };

        futures::executor::block_on(handle).map_err(|e: Error| e.into())
    }

    fn update_account(
        &mut self,
        address: &Address,
        balance: &U256,
        nonce: &U256,
    ) -> Result<(), tanour::error::Error> {
        let mut request = self.client.update_account_request();
        {
            let mut tmp = Vec::new();
            tmp.resize(32, 0);

            request.get().set_address(address.as_bytes());

            balance.to_little_endian(&mut tmp);
            request.get().set_balance(&tmp);

            nonce.to_little_endian(&mut tmp);
            request.get().set_nonce(&tmp);
        }
        let handle = async move {
            debug!("Try ot call `update_account` method in client");
            request.send().promise.await?;

            Ok(())
        };

        futures::executor::block_on(handle).map_err(|e: Error| e.into())
    }

    fn storage_at(&self, address: &Address, key: &H256) -> Result<H256, tanour::error::Error> {
        let mut request = self.client.storage_at_request();
        {
            request.get().set_address(address.as_bytes());
            request.get().set_key(key.as_bytes());
        }
        let handle = async move {
            debug!("Try ot call `storage_at` method in client");
            let result = request.send().promise.await?;
            let storage = result.get()?.get_storage()?;

            Ok(H256::from_slice(storage))
        };

        futures::executor::block_on(handle).map_err(|e: Error| e.into())
    }

    fn set_storage(
        &mut self,
        address: &Address,
        key: &H256,
        value: &H256,
    ) -> Result<(), tanour::error::Error> {
        let mut request = self.client.set_storage_request();
        {
            request.get().set_address(address.as_bytes());
            request.get().set_key(key.as_bytes());
            request.get().set_value(value.as_bytes());
        }
        let handle = async move {
            debug!("Try ot call `set_storage` method in client");
            request.send().promise.await?;

            Ok(())
        };

        futures::executor::block_on(handle).map_err(|e: Error| e.into())
    }

    fn block_hash(&self, _num: u64) -> Result<H256, tanour::error::Error> {
        Ok(H256::zero())
    }

    fn timestamp(&self) -> u64 {
        0
    }

    fn block_number(&self) -> u64 {
        0
    }

    fn block_author(&self) -> Result<Address, tanour::error::Error> {
        Err(tanour::error::Error::NotSupported)
    }

    fn difficulty(&self) -> Result<U256, tanour::error::Error> {
        Err(tanour::error::Error::NotSupported)
    }

    fn gas_limit(&self) -> Result<U256, tanour::error::Error> {
        Ok(U256::zero())
    }
}
