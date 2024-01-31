use std::{net::ToSocketAddrs, process::exit};
use tokio::signal::ctrl_c;

use actix_web::{App, HttpServer};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use common::event_capnp::event_interface;
use futures::AsyncReadExt;
use http_handler::home::home;
use tokio_util::sync::CancellationToken;

mod http_handler;
mod rpc_impl;
mod template_struct;

const RPC_HOST: Option<&str> = option_env!("RPC_HOST");
const HTTP_HOST: Option<&str> = option_env!("HTTP_HOST");

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Create an Arc to share shutdown signals between tasks
    let shutdown_signal = CancellationToken::new();
    let listen_shutdown_signal = shutdown_signal.clone();

    // Spawn a task to listen for SIGINT signals and notify on shutdown
    tokio::spawn(async move {
        match ctrl_c().await {
            Ok(()) => {
                shutdown_signal.cancel();
            }
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);

                exit(1);
            }
        }

        Ok::<(), std::io::Error>(())
    });

    let http_listen_address = HTTP_HOST
        .unwrap_or(&"[::]:8080")
        .to_socket_addrs()?
        .next()
        .expect("HTTP_HOST should be a valid address");

    let http_server = HttpServer::new(|| App::new().service(home))
        .bind(http_listen_address)?
        .run();

    // Spawn HTTP server on the default runtime
    tokio::spawn(http_server);

    let rpc_listen_address = RPC_HOST
        .unwrap_or(&"[::]:6969")
        .to_socket_addrs()?
        .next()
        .expect("RPC_HOST should be a valid address");

    let listener = tokio::net::TcpListener::bind(&rpc_listen_address).await?;
    let event_rpc: event_interface::Client =
        capnp_rpc::new_client(crate::rpc_impl::event_rpc::EventRPCImpl);

    println!("RPC server now listening at {}", rpc_listen_address);

    tokio::task::LocalSet::new()
        .run_until(async move {
            loop {
                tokio::select! {
                    _ = listen_shutdown_signal.cancelled() => {
                        break;
                    }
                    listen_result = listener.accept() => {
                        match listen_result {
                            Ok((stream, _)) => {
                                let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                                let network = twoparty::VatNetwork::new(
                                    reader,
                                    writer,
                                    rpc_twoparty_capnp::Side::Server,
                                    Default::default(),
                                );

                                let rpc_system = RpcSystem::new(Box::new(network), Some(event_rpc.clone().client));

                                tokio::task::spawn_local(rpc_system);
                            }
                            Err(err) => {
                                eprintln!("Error while opening listen on RPC server: {}", err);

                                exit(1);
                            }
                        }
                    }
                }
            }
        })
        .await;

    println!("Shutting down gracefully...");

    Ok(())
}
