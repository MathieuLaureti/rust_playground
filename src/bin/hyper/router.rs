use hyper::{Request, Response, body::Incoming};
use http_body_util::Full;
use hyper::body::Bytes;
use std::convert::Infallible;
use std::sync::Arc;
use crate::shared_states;

// This replaces your manual buffer logic
pub async fn router(request: Request<Incoming>, states: Arc<shared_states::AppState>) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = request.uri().path();
    let method = request.method();

    println!("{}",states.is_read_only_mode);

    // 1. Check if method is allowed (Security/Business logic)
    if method != hyper::Method::GET {
        return Ok(Response::builder()
            .status(405)
            .body(Full::new(Bytes::from("Method Not Allowed")))
            .unwrap());
    }

    // 2. Simple Router Logic
    match path {
        "/" => Ok(Response::new(Full::new(Bytes::from("Welcome to the Cooking API")))),
        "/recipes" => Ok(Response::new(Full::new(Bytes::from("Recipe List")))),
        _ => Ok(Response::builder()
                .status(404)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap()),
    }
}