use std::net::ToSocketAddrs;

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use common::event_capnp;
use futures::AsyncReadExt;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let backend_url = option_env!("LMK_BACKEND_URL")
        .unwrap_or("localhost:6969")
        .to_socket_addrs()?
        .next()
        .expect("Backend URL is not valid!");

    if args.len() < 3 {
        println!("lmk {}", VERSION.unwrap_or("unknown ver."));
        println!("usage: {} <IDENTIFIER> <SUMMARY> [BODY]", args[0]);

        return Ok(());
    }

    println!(
        "Sending '{}' to id '{}' with summary '{}' to backend at {}",
        args[2],
        args[1],
        args.get(3).unwrap_or(&"No summary :(".into()),
        backend_url
    );

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&backend_url).await?;
            let _ = stream.set_nodelay(true);

            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));

            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let event_rpc: event_capnp::event_interface::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(rpc_system);

            let mut req = event_rpc.trigger_request();
            let mut event_req = req.get().init_event();

            event_req.set_identifier(args[1].to_string());
            event_req.set_timestamp(common::timestamp::generate_now());

            let mut event_req_message = event_req.init_message();
            event_req_message.set_body(args[2].to_string());
            event_req_message.set_summary(args.get(3).unwrap_or(&"No summary :(".into()));

            req.send().promise.await?;

            Ok(())
        })
        .await
}
