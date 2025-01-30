use sqlx::{MySql, MySqlPool, mysql::MySqlPoolOptions};
use std::sync::Arc;
use crate::{db_connection::establish_connection, settings::Settings};

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub db_pool: Option<Arc<MySqlPool>>, // ✅ DB 연결이 실패할 경우 None을 허용
}

impl AppState {
    pub async fn new(settings: Settings) -> Self {
        let db_pool = establish_connection(&settings).await;
        Self {
            settings: Arc::new(settings),
            db_pool,
        }
    }
}
