use sqlx::MySqlPool;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::{adapter::reservation_adapter::ReservationAdapter, application::{port::{r#in::reservation_usecase::ReservationUseCase, out::{reservation_load_port::ReservationLoadPort, reservation_save_port::ReservationSavePort}}, reservation_service::ReservationService}, db_connection::establish_connection, grpc::grpc_service::ReservationGrpcService,  grpc_client::GrpcClients, infra::{db::reservation_repository::ReservationRepositoryImpl, web::reservation_controller::ReservationController}, settings::Settings};

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub db_pool: Arc<MySqlPool>,  
    pub reservation_service: Arc<dyn ReservationUseCase + Send + Sync>,
    pub reservation_controller: Arc<ReservationController>,
    pub grpc_server: Arc<ReservationGrpcService>,
    pub grpc_clients: Arc<Mutex<GrpcClients>>,
}

impl AppState {
    pub async fn new(settings: Settings) -> Self {
        let db_pool = establish_connection(&settings).await
            .expect("Failed to establish database connection."); 

            if let Err(err) = sqlx::migrate!().run(db_pool.as_ref()).await {
                eprintln!("❌ Migration failed: {}", err);
                std::process::exit(1);
            }
        println!("✅ Database migration completed!");

        let db_pool = Arc::new(db_pool);
        let repository = Arc::new(ReservationRepositoryImpl::new(Arc::clone(&db_pool)));
        let adapter = Arc::new(ReservationAdapter::new(repository.clone()));
        let save_port: Arc<dyn ReservationSavePort + Send + Sync> = adapter.clone();
        let load_port: Arc<dyn ReservationLoadPort + Send + Sync> = adapter.clone();
        let reservation_service: Arc<dyn ReservationUseCase + Send + Sync> = Arc::new(ReservationService::new(Arc::clone(&save_port), Arc::clone(&load_port)));
        //let reservation_service: Arc<dyn ReservationUseCase + Send + Sync> = Arc::new(ReservationService::new(adapter.clone())); 
        
        let grpc_clients = if let Ok(client) = GrpcClients::new("http://localhost:50052", "http://localhost:50053").await {
            println!("✅ Successfully connected to gRPC services");
            Arc::new(Mutex::new(client))
        } else {
            eprintln!("⚠️ Failed to connect to gRPC services. Proceeding with dummy clients.");
            Arc::new(Mutex::new(GrpcClients::dummy())) 
        };


        let reservation_controller = Arc::new(ReservationController::new(
            Arc::clone(&reservation_service),
            Arc::clone(&grpc_clients)
    ));
         // gRPC 서버 인스턴스 생성
         let grpc_server = Arc::new(ReservationGrpcService::new(Arc::clone(&reservation_service)));

         Self {
             settings: Arc::new(settings),
             db_pool: Arc::clone(&db_pool),
             reservation_service,
             reservation_controller,
             grpc_server, // gRPC 서버 추가
             grpc_clients
         }
    }
}
