extern crate prost_build;
use std::process;
#[allow(unused)]
use std::{fs, path::PathBuf};
/* use wasm_pack::{
    command::build::BuildOptions, command::build::Target, command::run_wasm_pack, command::Command,
};
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Build Protobufs ===");
    prost_build::compile_protos(&["proto/swarm.proto"], &["proto"])?;
    tonic_build::compile_protos("proto/api.proto")?;
    process::Command::new("protoc")
        .args([
            "--proto_path=proto",
            "--js_out=import_style=commonjs,binary:ui",
            "proto/web.proto",
        ])
        .output()?;

    println!("=== Build UI ===");
    fs::create_dir_all("ui/target/build/pkg")?;
    for file in fs::read_dir("ui")?
        .map(|entry| entry.unwrap().path())
        .filter(|path| match path.extension() {
            None => false,
            Some(ext) => ext == "html" || ext == "js",
        })
    {
        println!("Found Page {:#?}", file.clone());
        fs::copy(
            file.clone(),
            format!(
                "ui/dist/{}",
                file.file_name().expect("No file name").to_str().unwrap()
            ),
        )?;
    }
    /* let mut build_opt = BuildOptions::default();
    build_opt.path = Some(PathBuf::new().join("ui"));
    build_opt.out_dir = String::from("target/build/pkg");
    build_opt.target = Target::Web;
    run_wasm_pack(Command::Build(build_opt))?;
    */
    process::Command::new("npm")
        .args(["--prefix", "ui", "run", "build"])
        .output()?;
    Ok(())
}
