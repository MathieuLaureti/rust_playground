use tokio::net::{TcpListener};
use hyper_util::rt::TokioIo;
use hyper::server::conn::http1;

mod shared_states;
mod router;
mod gate_keeper;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    let states = shared_states::initialize_shared_state();

    println!("TCP Server running on 127.0.0.1:9999");

    loop {
        let (stream, _addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let states_clone = states.clone();
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