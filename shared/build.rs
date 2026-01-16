fn main() {
    prost_build::compile_protos(&["proto/tracker.proto"], &["proto/"]).unwrap();
}
