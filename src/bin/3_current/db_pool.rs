use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_optimized_pool(url: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5) // Strict limit for Raspberry Pi RAM 
        .min_connections(2) // Keep 2 ready to eliminate "cold start" latency 
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600)) // Release memory if inactive
        .connect(url)
        .await
        .expect("Failed to create pool")
}