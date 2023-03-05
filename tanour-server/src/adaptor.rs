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
    fn page_size(&self) -> Result<u32, tanour::error::Error> {
        let req = self.client.page_size_request();

        let handle = async move {
            debug!("Try ot call `page_size` method in client");
            let result = req.send().promise.await.unwrap(); //TODO: no unwrap
            result.get().unwrap().get_size() //TODO: no unwrap
        };

        Ok(futures::executor::block_on(handle))
    }

    fn read_page(&self, page_no: u32) -> Result<Vec<u8>, tanour::error::Error> {
        let mut req = self.client.read_page_request();
        req.get().set_page_no(page_no);

        let handle = async move {
            debug!("Try ot call `read_page` method in client");
            let result = req.send().promise.await.unwrap(); //TODO: no unwrap
            result.get().unwrap().get_data().unwrap().to_vec() //TODO: no unwrap
        };

        Ok(futures::executor::block_on(handle)) //TODO: no unwrap
    }

    fn write_page(&self, page_no: u32, data: &[u8]) -> Result<(), tanour::error::Error> {
        let mut req = self.client.write_page_request();
        req.get().set_page_no(page_no);
        req.get().set_data(data);

        let handle = async move {
            debug!("Try ot call `write_page` method in client");
            let result = req.send().promise.await.unwrap(); //TODO: no unwrap
            result.get().unwrap(); //TODO: no unwrap
        };

        futures::executor::block_on(handle);
        Ok(()) //TODO: no unwrap
    }

    fn exist(&self, address: &Address) -> Result<bool, tanour::error::Error> {
        let mut req = self.client.exists_request();
        req.get().set_address(address);

        let handle = async move {
            debug!("Try ot call `exists` method in client");
            let result = req.send().promise.await.unwrap(); //TODO: no unwrap
            result.get().unwrap().get_exist() //TODO: no unwrap
        };

        Ok(futures::executor::block_on(handle))
    }

    fn current_block_number(&self) -> u32 {
        todo!()
    }
}
