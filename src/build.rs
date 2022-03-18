extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["src/proto/swarm.proto"], &["src/proto"])?;
    tonic_build::compile_protos("src/proto/api.proto")?;
    Ok(())
}
