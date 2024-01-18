fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .output_path("src")
        .file("schema/event.capnp")
        .run()?)
}
