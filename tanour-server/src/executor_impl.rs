use crate::adaptor::BlockchainAdaptor;
use crate::tanour_capnp;
use crate::tanour_capnp::executor;
use capnp::capability::Promise;
use capnp::Error;
use capnp_rpc::pry;
use log::debug;
use tanour::address_from_bytes;
use tanour::contract::Params;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

pub struct ExecutorImpl;

impl executor::Server for ExecutorImpl {
    fn execute(
        &mut self,
        params: executor::ExecuteParams,
        mut results: executor::ExecuteResults,
    ) -> Promise<(), Error> {
        let (tx, mut rx) = oneshot::channel();

        tokio::task::spawn_local(async move {
            let provider_client = pry!(pry!(params.get()).get_provider());
            let transaction = pry!(pry!(params.get()).get_transaction());
            let adaptor = BlockchainAdaptor::new(provider_client);

            match pry!(transaction.get_action().which()) {
                tanour_capnp::transaction::action::Instantiate(reader) => {
                    let _msg = pry!(transaction.get_args());
                    let address = address_from_bytes(pry!(transaction.get_address()));
                    let code = pry!(reader.get_code());
                    let params1 = Params {
                        memory_limit_page: 1000,
                        metering_limit: 11100,
                    };

                    let _contract =
                        tanour::contract::Contract::new(Box::new(adaptor), &address, code, params1)
                            .unwrap(); // TODO:

                    // contract.call_instantiate(msg.clone()).unwrap();
                }
                tanour_capnp::transaction::action::Process(_reader) => {
                    todo!()
                }
                tanour_capnp::transaction::action::Query(_reader) => {
                    todo!()
                }
            };

            debug!("provider: {:?}", std::thread::current().id());

            tx.send(1).unwrap();
            Promise::<(), Error>::ok(())
        });
        debug!("executor: {:?}", std::thread::current().id());

        Promise::from_future(async move {
            loop {
                let msg = rx.try_recv();
                match msg {
                    Err(TryRecvError::Empty) => {}
                    Err(e) => {
                        return Err(Error::failed(format!("{e}")));
                    }
                    Ok(_result_data) => {
                        tokio::time::delay_for(std::time::Duration::from_millis(10_u64)).await;

                        let mut tmp = Vec::new();
                        tmp.resize(32, 0);

                        let _builder = results.get().get_result_data().unwrap();

                        // result_data.gas_left.to_little_endian(&mut tmp);
                        // builder.set_gas_left(0);
                        // builder.set_data(&result_data.data);
                        // builder.set_contract(&result_data.contract.as_bytes());

                        // TODO: Implement it later
                        //builder.set_logs();

                        break;
                    }
                };

                //print!(".");
                //tokio::task::yield_now().await;
                //tokio::time::delay_for(std::time::Duration::from_millis(10 as u64)).await;
            }

            Ok(())
        })
    }
}
