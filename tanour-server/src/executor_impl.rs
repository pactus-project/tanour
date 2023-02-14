use crate::adaptor::BlockchainAdaptor;
use crate::tanour_capnp;
use crate::tanour_capnp::executor;
use capnp::capability::Promise;
use capnp::Error;
use capnp_rpc::pry;
use log::debug;
use tanour::contract::{Contract, Params};
use tanour::{address_from_bytes, contract, Address};
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

// impl<'a> From<tanour_capnp::transaction::Reader<'a>>
//     for Result<tanour::transaction::Transaction, Error>
// {
//     fn from(reader: tanour_capnp::transaction::Reader<'a>) -> Self {
//         let sender = Address::from_slice(reader.get_sender()?);
//         let value = U256::from_little_endian(reader.get_value()?);
//         let gas = U256::from_little_endian(reader.get_gas()?);
//         let gas_price = U256::from_little_endian(reader.get_gas_price()?);
//         let args = reader.get_args()?.to_vec();
//         let action = match reader.get_action().which()? {
//             tanour_capnp::transaction::action::Create(create) => {
//                 let code = create.get_code()?.to_vec();

//                 let salt = H256::from_slice(create.get_salt()?);
//                 tanour::transaction::Action::Create(code, salt)
//             }
//             tanour_capnp::transaction::action::Call(call) => {
//                 let address = Address::from_slice(call.get_address()?);
//                 tanour::transaction::Action::Call(address)
//             }
//         };

//         Ok(tanour::transaction::Transaction {
//             sender: sender,
//             value: value,
//             gas: gas,
//             gas_price: gas_price,
//             action: action,
//             args: args,
//         })
//     }
// }

pub struct ExecutorImpl;

unsafe impl Send for tanour_capnp::provider::Client {}

impl executor::Server for ExecutorImpl {
    fn execute(
        &mut self,
        params: executor::ExecuteParams,
        mut results: executor::ExecuteResults,
    ) -> Promise<(), Error> {
        let (tx, mut rx) = oneshot::channel();

        let a = tokio::task::spawn_local(async move {
            let provider_client = pry!(pry!(params.get()).get_provider());
            let transaction = pry!(pry!(params.get()).get_transaction());
            let mut adaptor = BlockchainAdaptor::new(provider_client);

            match pry!(transaction.get_action().which()) {
                tanour_capnp::transaction::action::Instantiate(reader) => {
                    let msg = pry!(transaction.get_args());
                    let address = address_from_bytes(pry!(transaction.get_address()));
                    let owner = address_from_bytes(pry!(reader.get_owner()));
                    let code = pry!(reader.get_code());
                    let storage_size = reader.get_storage_size();
                    let valid_until = reader.get_valid_until();
                    let params1 = Params {
                        memory_limit_page: 1000,
                        metering_limit: 11100,
                        storage_path: ".".to_string(),
                    };

                    let contract = tanour::contract::Contract::create(
                        Box::new(adaptor),
                        &address,
                        storage_size,
                        valid_until,
                        owner,
                        code,
                        params1,
                    )
                    .unwrap(); // TODO:

                   // contract.call_instantiate(msg.clone()).unwrap();
                }
                tanour_capnp::transaction::action::Process(reader) => {
                    todo!()
                }
                tanour_capnp::transaction::action::Query(reader) => {
                    todo!()
                }
            };

            debug!("provider: {:?}", std::thread::current().id());

            tx.send(1).unwrap();
            return Promise::<(), Error>::ok(());
        });
        debug!("executor: {:?}", std::thread::current().id());

        return Promise::from_future(async move {
            loop {
                let msg = rx.try_recv();
                match msg {
                    Err(TryRecvError::Empty) => {}
                    Err(e) => {
                        return Err(Error::failed(format!("{}", e)));
                    }
                    Ok(result_data) => {
                        tokio::time::delay_for(std::time::Duration::from_millis(10 as u64)).await;

                        let mut tmp = Vec::new();
                        tmp.resize(32, 0);

                        let mut builder = results.get().get_result_data().unwrap();

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
                tokio::task::yield_now().await;
                //tokio::time::delay_for(std::time::Duration::from_millis(10 as u64)).await;
            }

            Ok(())
        });
    }
}
