use hyper::{Request, Response, Method, body::Incoming}; // Use Incoming for Hyper 1.0 [cite: 49]
use http_body_util::Full;                               // Helper to create a full body [cite: 35]
use hyper::body::Bytes;                                 // For zero-copy data handling [cite: 63, 66]
use std::convert::Infallible;                           // For the Result type where error is impossible
use std::sync::Arc;                                     // To share AppState across threads [cite: 46]
use std::option::Option;                    
// For handling optional values
// Assuming your AppState and router are in these locations
use crate::shared_states::AppState; 
use crate::router::router;
use crate::utils;

pub enum Route {
    Dishes, //retrieve dish list
    Ingredients, //retrieve ingredient list
    Recipes(i32), //there are dishes and recipes, dish contains multiple recipes, you must list recipes in a dish section
    Recipe(i32), //retrieve a single recipe by its ID
    Ingredient(i32), // retrieve single ingredient with details
}

fn extract_id(path: &str, prefix: &str) -> Option<i32> {
    let remainder = path.strip_prefix(prefix)?;
    if remainder.contains('/') { return None; }
    remainder.parse::<i32>().ok()
}
// recognize paths are /recipes/dishes, /recipes/recipes/{dish_id:int}, /recipes/recipe/{recipe_id:int},
// /match_checker/ingrdients, /match_checker/ingredient/{ingredient_id:int}
pub async fn gate_keeper(
    req: Request<Incoming>,
    state: Arc<AppState>
) -> Result<Response<Full<Bytes>>, Infallible> {
    // 1. Pre-processing (Security/Logging)
    if req.method() != Method::GET {
        return Ok(utils::build_response(405, "Method Not Allowed"));
    }
    let path: &str = req.uri().path();

    let route = match path {
        "/recipes/dishes" => Route::Dishes,
        "/match_checker/ingredients" => Route::Ingredients,
        p if p.starts_with("/recipes/recipes/") => {
            match extract_id(p, "/recipes/recipes/") {
                Some(id) => Route::Recipes(id),
                None => return Ok(utils::build_response(400, "Invalid Dish ID")),
            }
        }
        p if p.starts_with("/recipes/recipe/") => {
            match extract_id(p, "/recipes/recipe/") {
                Some(id) => Route::Recipe(id),
                None => return Ok(utils::build_response(400, "Invalid Recipe ID")),
            }
        }
        p if p.starts_with("/match_checker/ingredient") => {
            match extract_id(p, "/match_checker/ingredient") {
                Some(id) => Route::Ingredient(id),
                None => return Ok(utils::build_response(400, "Invalid Ingredient ID"))
            }
        }
        _ => return Ok(utils::build_response(404, "Not Found")),
    };
    // 2. Call the Router
    let response = router(route,state).await;
    
    // 3. Post-processing (Metrics for your comparative analysis)
    response
}
