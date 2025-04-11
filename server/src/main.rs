use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();


        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, tx, rx).await {
                eprintln!("Error en cliente {}: {}", addr, e);
            }
        });
    }
    
}

async fn handle_client(
    socket: TcpStream,  
    tx: broadcast::Sender<String>, 
    mut rx: broadcast::Receiver<String>) -> Result<(), Box<dyn std::error::Error>> {

    let socket = Arc::new(tokio::sync::Mutex::new(socket));
    let read_socket = Arc::clone(&socket);
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        let mut buf_len = [0; 4];
        loop {
            if read_socket.lock().await.read_exact(&mut buf_len).await.is_err() {
                break;
            }
            let len = u32::from_be_bytes(buf_len) as usize;

            let mut buf_msj = vec![0; len];
            if read_socket.lock().await.read_exact(&mut buf_msj).await.is_err() {
                break;
            }

            let msg = String::from_utf8_lossy(&buf_msj).to_string();

            let _ = tx_clone.send(msg.clone());
        }
    });

    let write_socket = Arc::clone(&socket);
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let msg_bytes = msg.as_bytes();
            let len = msg_bytes.len() as u32;
            let len_bytes = len.to_be_bytes();

            if write_socket.lock().await.write_all(&len_bytes).await.is_err() {
                break;
            }
            if write_socket.lock().await.write_all(msg_bytes).await.is_err() {
                break;
            }
        }
    });

    

    Ok(())

}
