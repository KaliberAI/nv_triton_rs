fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src") // you can change the generated code's location
        .compile(
            &["common/protobuf/grpc_service.proto"],
            &["common/protobuf"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
