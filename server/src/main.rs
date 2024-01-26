use std::net::ToSocketAddrs;

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use common::event_capnp::event_interface;
use futures::AsyncReadExt;

mod rpc_impl;
mod template_struct;

const HOST: Option<&str> = option_env!("HOST");

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_address = HOST
        .unwrap_or(&"[::]:6969")
        .to_socket_addrs()?
        .next()
        .expect("HOST should be a valid address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&listen_address).await?;
            let event_rpc: event_interface::Client =
                capnp_rpc::new_client(crate::rpc_impl::event_rpc::EventRPCImpl);

            println!("Server now listening at {}", listen_address);

            loop {
                let (stream, _) = listener.accept().await?;
                let _ = stream.nodelay()?;

                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let netowrk = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system = RpcSystem::new(Box::new(netowrk), Some(event_rpc.clone().client));

                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}
