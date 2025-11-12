fn main() {
    tonic_build::compile_protos("proto/sieve.proto").unwrap();
}
