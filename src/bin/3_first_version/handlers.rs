use crate::shared_states::AppState;
use std::sync::Arc;
use crate::db_models::{Dish,SearchRecipe,FullRecipe,MatchList,MatchChecker};
use crate::utils::serialize_to_bytes;
use hyper::body::Bytes;

pub async fn get_dish_list(states: Arc<AppState>) -> Bytes {
    let result = sqlx::query_as::<_, Dish>("SELECT id, name FROM dish")
        .fetch_all(&states.pool)
        .await;

    match result {
        Ok(dishes) => serialize_to_bytes(dishes, b"[]").await,
        Err(e) => {
            eprintln!("Database error: {}", e); 
            Bytes::from_static(b"[]")
        }
    }
}

pub async fn get_recipes_for_dish(dish_id: i32, states: Arc<AppState>) -> Bytes {
    let result = sqlx::query_as!(
        SearchRecipe,
        "SELECT id, name FROM recipe WHERE dish_id = $1",
        dish_id
    ).fetch_all(&states.pool).await;

    match result {
        Ok(recipes) => serialize_to_bytes(recipes, b"[]").await,
        Err(e) => {
            eprintln!("Database error: {}", e); 
            Bytes::from_static(b"[]")
        }
    }
}

pub async fn get_recipe_by_id(recipe_id: i32, states: Arc<AppState>) -> Bytes {
    let result = sqlx::query_scalar!(
        r#"
        SELECT json_build_object(
            'id', d.id,
            'name', d.name,
            'components', (
                SELECT json_agg(json_build_object(
                    'id', rc.id,
                    'name', rc.name,
                    'ingredients', (SELECT json_agg(i) FROM ingredient i WHERE i.component_id = rc.id),
                    'instructions', (SELECT json_agg(ins ORDER BY ins.step) FROM instruction ins WHERE ins.component_id = rc.id)
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
            tokio::task::spawn_blocking(move || {

                let mut buffer = Vec::with_capacity(4096); 

                if serde_json::to_writer(&mut buffer, &recipe_json).is_ok() {
                    Bytes::from(buffer)
                } else {
                    Bytes::from_static(b"{}")
                }
            })
            .await
            .unwrap_or_else(|_| Bytes::from_static(b"{}"))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Bytes::from_static(b"{}")
        }
    }
}

pub async fn get_ingredients(states: Arc<AppState>) -> Bytes {
    let result = sqlx::query_as::<_, Dish>("SELECT id, name FROM match_checker") // Fixed typo in 'titel'
        .fetch_all(&states.pool)
        .await;
    
    match result {
        Ok(data) => serialize_to_bytes(data, b"[]").await,
        Err(_) => Bytes::from_static(b"[]")
    }
}

pub async fn get_ingredient(ingredient_id: i32, states: Arc<AppState>) -> Bytes {
    let result = sqlx::query_as!(
        MatchChecker,
        "SELECT id, title, avoid, affinities, matches FROM match_checker WHERE id = $1",
        ingredient_id
    ).fetch_all(&states.pool).await;

    match result {
        Ok(data) => serialize_to_bytes(data, b"[]").await,
        Err(_) => Bytes::from_static(b"[]")
    }
}
