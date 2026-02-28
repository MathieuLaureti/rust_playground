use std::sync::Arc;

use tokio::net::{TcpListener};
use hyper_util::rt::TokioIo;
use hyper::server::conn::http1;

// The goal of this Project is to implement a simple but highly optimized custom HTTP server 
// it uses Low Level libraries and techniques to achieve high performance, such as:
// - Hyper for HTTP parsing and response generation [cite: 49, 63, 66]
// - Tokio for async runtime and TCP handling [cite: 46]
// - http_body_util for efficient body handling [cite: 35]
// The server will handle three routes:
// - GET /dishes: returns a list of dishes (static response for simplicity)
// - GET /recipes/{dish_id}: returns recipes for a specific dish (dynamic response based on dish_id)
// - GET /recipe/{recipe_id}: returns a single recipe by its ID (dynamic response based on recipe_id)
// The server will also include a shared state to demonstrate how to manage application 
// and aim to be a reference implementation for building high-performance HTTP servers 
// in Rust, showcasing efficient use of libraries.
// state across requests, and a gate_keeper function to handle routing and pre-processing of requests.

mod shared_states;
// For shared application state management
// Creating a AppState struct to hold shared data
// Shared state is initialized once and passed to handlers
mod gate_keeper;
// The gate_keeper module will handle routing and pre-processing of requests
// it act as a middleware to check method, parse path and call the router
// But it is not a full middleware, it is more of a request handler
// this only handles GET requests and recognizes specific staticly defined paths
// It is not a reusable middleware, it is specific to this application and its routes
mod router;
// The router doesnt handle much logic, it just matches the route and returns a response
// based on the route and parameters staticly defined in the gate_keeper
// it is not a reusable router, it is specific to this application and its routes
mod handlers;
// The handlers module will contain the actual logic for handling each route
mod db_models;
// The db_models module will contain the data models
mod db_pool;
// The db_pool module will handle the database connection pool
mod utils;
// Utility functions for building responses and other common tasks
// this module currently handles :
// - Building HTTP responses with a given status and body text

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9999").await?;
    let states = shared_states::initialize_shared_state().await;

    eprintln!("TCP Server running on 0.0.0.0:9999");

    loop {
        let (stream, _addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let states_clone = Arc::clone(&states);
              // Multi-threaded execution
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, hyper::service::service_fn(move |req| {
                    gate_keeper::gate_keeper(req, states_clone.clone())
                }))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
