use crate::shared_states::AppState;
use std::sync::Arc;
use crate::db_models::{Dish,SearchRecipe,FullRecipe,MatchList,MatchChecker};

// /recipes/dishes
pub async fn get_dish_list(states: Arc<AppState>) -> String {
    let result = sqlx::query_as::<_, Dish>("SELECT id, name FROM dish")
        .fetch_all(&states.pool)
        .await;

    match result {
        Ok(dishes) => {
            serde_json::to_string(&dishes).unwrap_or_else(|_| "[]".to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e); 
            "[]".to_string() 
        }
    }
}
// /recipes/recipes/{dish_id}
pub async fn get_recipes_for_dish(dish_id: i32, states: Arc<AppState>) -> String {
    let result = sqlx::query_as!(
        SearchRecipe,
        "SELECT id, name FROM recipe WHERE dish_id = $1",
        dish_id
    ).fetch_all(&states.pool).await;
    match result {
        Ok(recipes) => {
            serde_json::to_string(&recipes).unwrap_or_else(|_| "[]".to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e); 
            "[]".to_string() 
        }
    }
}
// /recipes/recipe/{recipe_id}
pub async fn get_recipe_by_id(recipe_id: i32, states: Arc<AppState>) -> String {
    let result = sqlx::query_scalar!(
        r#"
        SELECT json_build_object(
            'id', d.id,
            'name', d.name,
            'components', (
                SELECT json_agg(json_build_object(
                    'id', rc.id,
                    'name', rc.name,
                    'ingredients', (
                        SELECT json_agg(i) 
                        FROM ingredient i 
                        WHERE i.component_id = rc.id
                    ),
                    'instructions', (
                        SELECT json_agg(ins ORDER BY ins.step) 
                        FROM instruction ins 
                        WHERE ins.component_id = rc.id
                    )
                ))
                FROM recipe r
                JOIN recipe_component rc ON rc.recipe_id = r.id
                WHERE r.dish_id = d.id
            )
        ) as "recipe!"
        FROM dish d
        WHERE d.id = $1
        "#,
        recipe_id
    )
    .fetch_one(&states.pool)
    .await;

    match result {
        Ok(recipe_json) => {
            serde_json::to_string(&recipe_json).unwrap_or_else(|_| "{}".to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            "{}".to_string()
        }
    }
}
// /match_checker/ingredients
pub async fn get_ingredients(states: Arc<AppState>) -> String {
    let result = sqlx::query_as::<_, Dish>("SELECT id, titel FROM match_checker")
        .fetch_all(&states.pool)
        .await;
    
    match result {
        Ok(match_checker_json) => {
            serde_json::to_string(&match_checker_json).unwrap_or_else(|_| "{}".to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            "{}".to_string()
        }
    }
}
// /match_checker/ingredient/{ingredient_id}
pub async fn get_ingredient(ingredient_id: i32, states: Arc<AppState>) -> String {
    let result = sqlx::query_as!(
        MatchChecker,
        "SELECT id, title, avoid, affinities, matches FROM match_checker WHERE id = $1",
        ingredient_id
    ).fetch_all(&states.pool).await;
    match result {
        Ok(recipes) => {
            serde_json::to_string(&recipes).unwrap_or_else(|_| "[]".to_string())
        }
        Err(e) => {
            eprintln!("Database error: {}", e); 
            "[]".to_string() 
        }
    }
}
