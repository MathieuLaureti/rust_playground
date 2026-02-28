use std::sync::Arc;

use hyper_util::client::pool;

use crate::db_pool::create_optimized_pool;
//use tokio::sync::Mutex;

pub struct AppState {
    // Mutable shared state for connection count
    //pub connection_count: Mutex<u64>,
    // Immutable shared state for read-only mode
    //pub is_read_only_mode: bool,
    pub pool: sqlx::PgPool,
    //pub database_url: String,
}

pub async fn initialize_shared_state() -> Arc<AppState> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_optimized_pool(&database_url).await;
    Arc::new(AppState {
        //connection_count: Mutex::new(0),
        // Read-only mode is determined by the TEST environment variable
        //is_read_only_mode: std::env::var("TEST").unwrap_or_else(|_| "false".to_string()) == "true",
        pool: pool,
        }
    )
}