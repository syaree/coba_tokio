use tokio::io::{Result};
use tokio::net::TcpListener;
use tokio::prelude::io::AsyncReadExt;


#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut stream, addr) = listener.accept().await?;
        let mut buff = [0; 1024];

        println!("Client connected from {}", addr.to_string());

        tokio::task::spawn(async move {
            loop {
                match stream.read(&mut buff).await {
                    Ok(len) if len == 0 => return,
                    Ok(len) => {
                        let message = String::from_utf8_lossy(&buff[..len]);
                        println!("Receive: {:?}", message.trim());

                        if message.trim() == "?q" { return }
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        return
                    }
                }
            }
        });
    }
}
