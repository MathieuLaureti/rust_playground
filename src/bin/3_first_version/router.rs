use hyper::{Request, Response, body::Incoming};
use http_body_util::Full;
use hyper::body::Bytes;
use std::convert::Infallible;
use std::sync::Arc;
use crate::shared_states;
use crate::gate_keeper;
use crate::handlers::{get_dish_list, get_recipes_for_dish, get_recipe_by_id, get_ingredient, get_ingredients};

pub async fn router(route: gate_keeper::Route, states: Arc<shared_states::AppState>) -> Result<Response<Full<Bytes>>, Infallible> {

    match route {
        gate_keeper::Route::Dishes => Ok(
            Response::new(Full::new(Bytes::from(get_dish_list(states).await)))
        ),
        gate_keeper::Route::Ingredients => Ok(
            Response::new(Full::new(Bytes::from(get_ingredients(states).await)))
        ),
        gate_keeper::Route::Recipes(id) => Ok(
            Response::new(Full::new(Bytes::from(get_recipes_for_dish(id,states).await)))
        ),
        gate_keeper::Route::Recipe(id) => Ok(
            Response::new(Full::new(Bytes::from(get_recipe_by_id(id,states).await)))
        ),
        gate_keeper::Route::Ingredient(id) => Ok(
            Response::new(Full::new(Bytes::from(get_ingredient(id,states).await)))
        )

    }
}
