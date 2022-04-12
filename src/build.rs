extern crate prost_build;
use std::{fs, path::PathBuf};
use wasm_pack::{
    command::build::BuildOptions, command::build::Target, command::run_wasm_pack, command::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Build Protobufs ===");
    prost_build::compile_protos(&["src/proto/swarm.proto"], &["src/proto"])?;
    tonic_build::compile_protos("src/proto/api.proto")?;
    println!("=== Build UI ===");
    fs::create_dir_all("ui/target/build/pkg")?;
    for file in fs::read_dir("ui")?
        .map(|entry| entry.unwrap().path())
        .filter(|path| match path.extension() {
            None => false,
            Some(ext) => ext == "html",
        })
    {
        println!("Found Page {:#?}", file.clone());
        fs::copy(
            file.clone(),
            format!(
                "ui/target/build/{}",
                file.file_name().expect("No file name").to_str().unwrap()
            ),
        )?;
    }
    let mut build_opt = BuildOptions::default();
    build_opt.path = Some(PathBuf::new().join("ui"));
    build_opt.out_dir = String::from("target/build/pkg");
    build_opt.target = Target::Web;
    run_wasm_pack(Command::Build(build_opt))?;
    Ok(())
}
