
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");

    stream.write_all(b"Hello, server!").await?;
    println!("Message sent to server");

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("Received from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())

}
