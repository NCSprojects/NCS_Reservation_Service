use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::settings::Settings;

pub async fn establish_connection(settings: &Settings) -> Option<Arc<MySqlPool>> {
    let mut retries = 5;
    let mut db_pool = None;

    while retries > 0 {
        match MySqlPoolOptions::new()
            .max_connections(20) // ✅ 최대 연결 수 증가
            .acquire_timeout(Duration::from_secs(5)) // ✅ 5초 제한
            .connect(&settings.database_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Successfully connected to MySQL");
                db_pool = Some(Arc::new(pool));
                break;
            }
            Err(err) => {
                eprintln!("⚠️ Failed to connect to MySQL: {}. Retrying in 3s...", err);
                retries -= 1;
                sleep(Duration::from_secs(3)).await;
            }
        }
    }

    if db_pool.is_none() {
        eprintln!("❌ Failed to connect to MySQL after retries. Server will run without DB.");
    }

    db_pool
}
