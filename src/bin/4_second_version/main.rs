use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::net::{TcpListener};
use hyper_util::rt::{TokioIo,TokioTimer};
use hyper::server::conn::http1;
use std::net::SocketAddr;
use socket2::{Domain, Protocol, Socket, Type};

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

fn create_reusable_listener(addr: SocketAddr) -> TcpListener {
    let domain = if addr.is_ipv6() { Domain::IPV6 } else { Domain::IPV4 };
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP)).unwrap();

    socket.set_reuse_port(true).unwrap();
    socket.set_reuse_address(true).unwrap();
    socket.bind(&addr.into()).unwrap();
    socket.listen(8192).unwrap();

    let std_listener: std::net::TcpListener = socket.into();
    std_listener.set_nonblocking(true).unwrap();
    TcpListener::from_std(std_listener).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::]:6669".parse()?;
    let states = shared_states::initialize_shared_state().await;
    let cores = num_cpus::get();
    
    let semaphore = Arc::new(Semaphore::new(20_000));
    let acceptor_count = cores.min(8);

    eprintln!("Server launched. Listening on port {}", addr.port());

    for _ in 0..acceptor_count {
        let s_clone = Arc::clone(&states);
        let sem_clone = Arc::clone(&semaphore);
        let listener = create_reusable_listener(addr);

        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else { continue };

                let Ok(permit) = sem_clone.clone().acquire_owned().await else { continue };

                let s = Arc::clone(&s_clone);
                
                tokio::spawn(async move {
                    let _ = stream.set_nodelay(true);
                    let io = TokioIo::new(stream);
                    
                    let _ = http1::Builder::new()
                        .keep_alive(true)
                        .timer(TokioTimer::new())
                        .serve_connection(io, hyper::service::service_fn(move |req| {
                            gate_keeper::gate_keeper(req, Arc::clone(&s))
                        }))
                        .await;
                    
                    drop(permit);
                });
            }
        });
    }

    std::future::pending::<()>().await;
    Ok(())
}
