
fn main() {
    ::capnpc::CompilerCommand::new().file("tanour.capnp").run().unwrap();
}