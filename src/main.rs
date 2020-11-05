use tokio::io::{Result, AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpListener;


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

                        if let Err(e) = stream.write_all(&buff[..len]).await {
                            eprintln!("Error: {:?}", e);
                            return
                        }
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
