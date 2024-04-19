fn main() {
    prost_build::Config::new()
        .out_dir("src/proto/build")
        .compile_protos(&["src/proto/data.proto"], &["src/"])
        .unwrap();
}
