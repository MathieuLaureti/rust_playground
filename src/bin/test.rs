use dotenvy::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file into the environment
    dotenv().expect(".env file not found");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = create_optimized_pool(&database_url).await;
    
    // ... rest of your application logic
    Ok(())
}

pub async fn create_optimized_pool(url: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .connect(url)
        .await
        .expect("Failed to create pool")
}