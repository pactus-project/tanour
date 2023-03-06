use crate::adaptor::BlockchainAdaptor;
use crate::tanour_capnp;
use crate::tanour_capnp::executor;
use capnp::capability::Promise;
use capnp::Error;
use capnp_rpc::pry;
use tanour::address_from_bytes;
use tanour::contract::Params;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

pub struct ExecutorImpl;

// TODO: ??? why ???
#[allow(clippy::async_yields_async)]
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
            let msg = pry!(transaction.get_args());
            let address = address_from_bytes(pry!(transaction.get_address()));
            let code = pry!(transaction.get_code());
            let params = Params {
                memory_limit_page: 1000,
                metering_limit: 11100,
            };

            let mut contract =
                tanour::contract::Contract::new(Box::new(adaptor), &address, code, params).unwrap(); // TODO: no unwrap

            let res = match pry!(transaction.get_action().which()) {
                tanour_capnp::transaction::action::Instantiate(_) => {
                    contract.call_instantiate(msg).unwrap() // TODO: no unwrap
                }
                tanour_capnp::transaction::action::Process(_) => {
                    contract.call_process(msg).unwrap() // TODO: no unwrap
                }
                tanour_capnp::transaction::action::Query(_) => {
                    contract.call_query(msg).unwrap() // TODO: no unwrap
                }
            };

            tx.send(res).unwrap(); // TODO: no unwrap
            Promise::<(), Error>::ok(())
        });

        Promise::from_future(async move {
            loop {
                let msg = rx.try_recv();
                match msg {
                    Err(TryRecvError::Empty) => {}
                    Err(e) => {
                        return Err(Error::failed(format!("{e}")));
                    }
                    Ok(result_data) => {
                        tokio::time::sleep(std::time::Duration::from_millis(10_u64)).await;

                        let mut builder = results.get().get_result_data().unwrap();
                        builder.set_data(&result_data);

                        break;
                    }
                };

                tokio::task::yield_now().await
            }

            Ok(())
        })
    }
}
