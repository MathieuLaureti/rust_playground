use crate::shared_states::AppState;
use std::sync::Arc;
use crate::db_models::Dish;


// /dishes -> get_dish_list
pub async fn get_dish_list(states: Arc<AppState>) -> String {
    // 1. Access the pool directly from your shared state object
    let result = sqlx::query_as::<_, Dish>("SELECT id, name FROM dish")
        .fetch_all(&states.pool) // Using the pool you initialized at the start
        .await;

    match result {
        Ok(dishes) => {
            // 2. Deep-dive JSON: Manual serialization
            // Instead of Axum's 'Json' wrapper, we use serde_json directly
            serde_json::to_string(&dishes).unwrap_or_else(|_| "[]".to_string())
        }
        Err(e) => {
            // 3. Error Management: Log for security/monitoring but return clean output
            eprintln!("Database error: {}", e); 
            "[]".to_string() 
        }
    }
}

pub async fn get_recipes_for_dish(dish_id: i32, states: Arc<AppState>) -> String {
    format!("Recipes for Dish ID: {}", dish_id)
}

pub async fn get_recipe_by_id(recipe_id: i32, states: Arc<AppState>) -> String {
    format!("Single Recipe ID: {}", recipe_id)
}