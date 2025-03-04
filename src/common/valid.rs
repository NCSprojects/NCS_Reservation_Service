use actix_web::HttpResponse;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::grpc_client::GrpcClients;

/// gRPC 토큰 검증을 수행하는 공통 함수
pub async fn validate_user_token(
    grpc_clients: Arc<Mutex<GrpcClients>>, 
    token: &str
) -> Result<String, HttpResponse> {
    let mut grpc_clients = grpc_clients.lock().await;

    match grpc_clients.validate_token(token.to_string()).await {
        Ok(Some(user_id)) => {
            println!("Received user_id from gRPC: {}", user_id);
            Ok(user_id)
        },
        Ok(None) => {
            println!("gRPC returned None for user_id!");
            Err(HttpResponse::Unauthorized().json("Invalid Token"))
        },
        Err(err) => {
            println!("gRPC call failed: {}", err);
            Err(HttpResponse::InternalServerError().json("Auth Service Error"))
        }
    }
}