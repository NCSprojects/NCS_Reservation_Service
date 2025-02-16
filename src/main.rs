use std::{net::TcpListener, sync::Arc};

use reservation_msservice::{grpc_server::run_grpc_server, settings::Settings, startup::run, state::AppState};
use tokio::join;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("❌ Failed to load settings");
    let state = Arc::new(AppState::new(settings).await);
    
    let listener = TcpListener::bind(format!("{}:{}", state.settings.server_host, state.settings.server_port))?;
    let actix_server = run(listener, Arc::clone(&state))?;

    // ✅ gRPC 서버 실행 (비동기)
    let grpc_server = run_grpc_server(Arc::clone(&state));

    // ✅ 두 서버를 동시에 실행 (Actix + gRPC)
    join!(actix_server, grpc_server);

    Ok(())
}
