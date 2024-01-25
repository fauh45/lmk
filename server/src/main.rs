use capnp::capability::Promise;
use capnp_rpc::pry;

// Still not sure about this (?)
struct Main {}

impl common::event_capnp::event_interface::Server for Main {
    fn trigger(
        &mut self,
        params: common::event_capnp::event_interface::TriggerParams,
        _: common::event_capnp::event_interface::TriggerResults,
    ) -> capnp::capability::Promise<(), capnp::Error> {
        let req = pry!(pry!(params.get()).get_event());

        let identifier = pry!(pry!(req.get_identifier()).to_str());
        let timestamp = req.get_timestamp();
        let message = req.get_message();

        // TODO: Handle when being called

        Promise::ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
