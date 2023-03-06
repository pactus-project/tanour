pub mod tanour_capnp {
    include!(concat!(env!("OUT_DIR"), "/tanour_capnp.rs"));
}
mod adaptor;
mod executor_impl;

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use executor_impl::ExecutorImpl;
use futures::{AsyncReadExt, TryFutureExt};
use std::net::ToSocketAddrs;
use tanour_capnp::executor;

use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} HOST:PORT", args[0]);
        return Ok(());
    }

    let addr = args[1]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = TcpListener::bind(&addr).await?;

            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let executor_impl = ExecutorImpl {};
                let executor_client: executor::Client = capnp_rpc::new_client(executor_impl);
                let rpc_system = RpcSystem::new(Box::new(network), Some(executor_client.client));

                tokio::task::spawn_local(
                    rpc_system.map_err(|err| log::error!("rpc_system error : {err}")),
                );
            }
        })
        .await
}
