use capnp::capability::Promise;
use capnp_rpc::pry;
use log::debug;

pub struct EventRPCImpl;

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

        debug!(target: "rpc_server",
            "Id: {}\nTimestamp: {}\nMessage:\n\tSummary: {}\n\tBody: {}",
            identifier, timestamp, message_summary, message_body
        );

        // TODO: Handle when being called

        Promise::ok(())
    }
}
