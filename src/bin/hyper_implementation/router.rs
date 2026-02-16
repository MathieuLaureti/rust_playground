use hyper::{Request, Response, body::Incoming};
use http_body_util::Full;
use hyper::body::Bytes;
use std::convert::Infallible;
use std::sync::Arc;
use crate::shared_states;
use crate::gate_keeper;
// This replaces your manual buffer logic
pub async fn router(route: gate_keeper::Route, states: Arc<shared_states::AppState>) -> Result<Response<Full<Bytes>>, Infallible> {

    match route {
        gate_keeper::Route::Dishes => Ok(Response::new(Full::new(Bytes::from("Welcome to the Cooking API")))),
        gate_keeper::Route::Recipes(dish_id) => Ok(Response::new(Full::new(Bytes::from(format!("Recipes for Dish ID: {}", dish_id))))),
        gate_keeper::Route::Recipe(id) => Ok(Response::new(Full::new(Bytes::from(format!("Single Recipe ID: {}", id))))),
    }
}