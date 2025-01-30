use std::{net::TcpListener, sync::Arc};

use reservation_msservice::{settings::Settings, state::AppState, startup::run};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("❌ Failed to load settings");
    let state = Arc::new(AppState::new(settings).await);
    
    let listener = TcpListener::bind(format!("{}:{}", state.settings.server_host, state.settings.server_port))?;
    let server = run(listener, Arc::clone(&state))?;

    server.await
}
