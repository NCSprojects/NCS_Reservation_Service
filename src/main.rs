use std::{net::TcpListener, sync::Arc};

use reservation_msservice::{error::server_error::ServerError, grpc_server::run_grpc_server, settings::Settings, startup::run, state::AppState};
use tokio::try_join;


#[actix_web::main]
async fn main() -> Result<(), ServerError>  /*std::io::Result<()>*/ {
    let settings = Settings::new().expect("❌ Failed to load settings");
    let state = Arc::new(AppState::new(settings).await);
    
    let listener = TcpListener::bind(format!("{}:{}", state.settings.server_host, state.settings.server_port))?;
    let actix_server = async {
        run(listener, Arc::clone(&state))?
            .await
            .map_err(ServerError::Io)
    };

    let grpc_server = async {
        run_grpc_server(Arc::clone(&state))
            .await
            .map_err(ServerError::Tonic)
    };

    try_join!(actix_server, grpc_server)?;

    Ok(())
}
