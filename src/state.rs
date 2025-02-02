use sqlx::{MySqlPool};
use std::sync::Arc;
use crate::{adapter::reservation_adapter::ReservationAdapter, application::{port::r#in::reservation_usecase::ReservationUseCase, reservation_service::{self, ReservationService}}, db_connection::establish_connection, infra::{db::reservation_repository::ReservationRepositoryImpl, web::reservation_controller::ReservationController}, settings::Settings};

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub db_pool: Arc<MySqlPool>,  
    pub reservation_service: Arc<dyn ReservationUseCase + Send + Sync>,
    pub reservation_controller: Arc<ReservationController>, 
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
        let reservation_service: Arc<dyn ReservationUseCase + Send + Sync> = Arc::new(ReservationService::new(adapter.clone())); 
        let reservation_controller = Arc::new(ReservationController::new(Arc::clone(&reservation_service)));
        Self {
            settings: Arc::new(settings),
            db_pool: Arc::clone(&db_pool),
            reservation_service,
            reservation_controller,
        }
    }
}
