use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use common::event_capnp::event_interface;
use futures::AsyncReadExt;
use http_handler::home::home;

mod http_handler;
mod rpc_impl;
mod template_struct;

const RPC_HOST: Option<&str> = option_env!("RPC_HOST");
const HTTP_HOST: Option<&str> = option_env!("HTTP_HOST");

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let http_listen_address = HTTP_HOST
        .unwrap_or(&"[::]:8080")
        .to_socket_addrs()?
        .next()
        .expect("HTTP_HOST should be a valid address");

    // Not working! I think I might need to make it run boh on different thread?
    let http_server = HttpServer::new(|| App::new().service(home))
        .bind(http_listen_address)?
        .run();

    tokio::spawn(http_server);

    tokio::task::LocalSet::new()
        .run_until(async move {
            let rpc_listen_address = RPC_HOST
                .unwrap_or(&"[::]:6969")
                .to_socket_addrs()?
                .next()
                .expect("RPC_HOST should be a valid address");

            let listener = tokio::net::TcpListener::bind(&rpc_listen_address).await?;
            let event_rpc: event_interface::Client =
                capnp_rpc::new_client(crate::rpc_impl::event_rpc::EventRPCImpl);

            println!("RPC server now listening at {}", rpc_listen_address);

            loop {
                let (stream, _) = listener.accept().await?;
                let _ = stream.nodelay()?;

                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system = RpcSystem::new(Box::new(network), Some(event_rpc.clone().client));

                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}
