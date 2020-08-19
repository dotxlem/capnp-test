extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("proto/iomod.capnp")
        .output_path("src")
        .src_prefix("proto")
        .run()
        .unwrap();
}
