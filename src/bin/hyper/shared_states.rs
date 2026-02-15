use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    // Mutable shared state for connection count
    pub connection_count: Mutex<u64>,
    // Immutable shared state for read-only mode
    pub is_read_only_mode: bool,
}

fn initialize_shared_state() -> Arc<AppState> {
    dotenvy::dotenv().ok();
    Arc::new(AppState {
        connection_count: Mutex::new(0),
        is_read_only_mode: false,
        }
    )
}