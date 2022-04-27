extern crate prost_build;
extern crate protoc_rust;
use std::process;
#[allow(unused)]
use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Build Protobufs ===");
    prost_build::compile_protos(&["proto/swarm.proto"], &["proto"])?;
    tonic_build::compile_protos("proto/api.proto")?;
    protoc_rust::Codegen::new()
        .out_dir("ui/src/proto")
        .inputs(&["proto/web.proto"])
        .include("proto")
        .customize(protoc_rust::Customize {
            carllerche_bytes_for_bytes: Some(true),
            carllerche_bytes_for_string: Some(true),
            ..Default::default()
        })
        .run()
        .expect("Running protoc failed.");

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
