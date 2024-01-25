use std::net::ToSocketAddrs;

use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use common::event_capnp::event_interface;
use futures::AsyncReadExt;

mod template_struct;

const HOST: Option<&str> = option_env!("HOST");

struct EventRPCImpl;

impl common::event_capnp::event_interface::Server for EventRPCImpl {
    fn trigger(
        &mut self,
        params: common::event_capnp::event_interface::TriggerParams,
        _: common::event_capnp::event_interface::TriggerResults,
    ) -> capnp::capability::Promise<(), capnp::Error> {
        let req = pry!(pry!(params.get()).get_event());

        let identifier = pry!(pry!(req.get_identifier()).to_str());
        let timestamp = req.get_timestamp();
        let message = pry!(req.get_message());

        let message_summary = pry!(pry!(message.get_summary()).to_str());
        let message_body = pry!(pry!(message.get_body()).to_str());

        println!("---");
        println!(
            "Id: {}\nTimestamp: {}\nMessage:\n\tSummary: {}\n\tBody: {}",
            identifier, timestamp, message_summary, message_body
        );
        println!("---");

        // TODO: Handle when being called

        Promise::ok(())
    }
}

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
            let event_rpc: event_interface::Client = capnp_rpc::new_client(EventRPCImpl);

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
