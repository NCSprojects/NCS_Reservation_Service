use tonic::transport::Server;
use crate::{grpc::grpc_service::ReservationGrpcService, reservation_proto::reservation_service_server::ReservationServiceServer};
use std::sync::Arc;
use crate::state::AppState;
use tokio::task;

pub async fn run_grpc_server(state: Arc<AppState>) {
    let addr = format!("{}:{}", state.settings.grpc_host, state.settings.grpc_port)
        .parse()
        .unwrap();

    let service = ReservationGrpcService::new(Arc::clone(&state.reservation_service)); // gRPC 서비스 인스턴스 생성

    println!("✅ gRPC Server running at {}", addr);

    Server::builder()
        .add_service(ReservationServiceServer::new(service))
        .serve(addr)
        .await
        .expect("gRPC server failed");
}

pub fn spawn_grpc_server(state: Arc<AppState>) {
    task::spawn(run_grpc_server(state));
}