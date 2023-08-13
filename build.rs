use std::{path::PathBuf, env};

fn main() {
    
    let out_dir: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap());

    // sb event
    let bidask = "./proto/services/exchange/bidask.proto";
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .out_dir("./src/models/")
        .compile(&[bidask], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    // grpc 
    let rataes_saver_grpc = "./proto/services/rates_saver/grpc/rates_saver.proto";
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .out_dir("./src/endpoints/grpc")
        .compile(&[rataes_saver_grpc], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", bidask);
    println!("cargo:rerun-if-changed={}", rataes_saver_grpc);

}