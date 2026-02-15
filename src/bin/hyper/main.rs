use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
mod shared_states;

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
            let _content = String::from_utf8_lossy(&buffer[..n]);
            //println!("--- Request Content ---\n{}", _content);
            
            // Simulating work
            sleep(Duration::from_millis(5)).await;
        }
        Err(e) => eprintln!("Failed to read: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    let states = Arc::new(Mutex::new(ConnectionCount { connection_count: 0 }));

    println!("TCP Server running on 127.0.0.1:9999");

    loop {
        let (socket, address) = listener.accept().await?;
        let states_clone = Arc::clone(&states);

        // Multi-threaded execution
        tokio::spawn(async move {
            process(socket, address, stats_clone).await
        });
    }
}