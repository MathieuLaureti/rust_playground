use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

// Concepts :
// - TCP Server using Tokio TcpListener function
// - Handling multiple connections concurrently using Tokio's async/await and spawn
// - Managing shared state across connections using Arc and Mutex
// - Socket management and reading data from the socket with TcpStream
// Create a TCP server that listens on a specified port, accepts incoming connections, and reads data from the socket. 
// The server should be able to handle multiple connections concurrently and manage shared state across connections using Arc and Mutex. 
// Additionally, implement basic logging of incoming connections and the data received from each connection.

// Shared state example
struct Stats {
    connection_count: u64,
}

async fn process(mut socket: TcpStream, address: std::net::SocketAddr, stats: Arc<Mutex<Stats>>) {
    let mut buffer = [0; 1024];
    
    // Increment shared state safely
    {
        let mut s = stats.lock().await;
        s.connection_count += 1;
        println!("Connection #{} from: {}", s.connection_count, address);
    }

    match socket.read(&mut buffer).await {
        Ok(0) => return, 
        Ok(n) => {
            let content = String::from_utf8_lossy(&buffer[..n]);
            println!("--- Request Content ---\n{}", content);
            
            // Simulating work
            sleep(Duration::from_millis(5)).await;
        }
        Err(e) => eprintln!("Failed to read: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    let stats = Arc::new(Mutex::new(Stats { connection_count: 0 }));

    println!("TCP Server running on 127.0.0.1:9999");

    loop {
        let (socket, address) = listener.accept().await?;
        let stats_clone = Arc::clone(&stats);

        // Multi-threaded execution
        tokio::spawn(async move {
            process(socket, address, stats_clone).await
        });
    }
}