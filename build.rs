fn main() {
    println!("Starting gRPC proto compilation...");

    match tonic_build::configure()
        .build_server(true)
        .compile_protos( &["proto/auth.proto", "proto/user.proto" ,"proto/reservation.proto","proto/reservationfcm.proto"], // 여러 proto 파일을 포함
        &["proto"]) 
    {
        Ok(_) => println!("gRPC proto files compiled successfully!"),
        Err(e) => panic!("Failed to compile gRPC definitions: {:?}", e),
    }
}