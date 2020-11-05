use std::error::Error;
use tokio::net::TcpStream;
use tokio::prelude::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = TcpStream::connect("127.0.0.1:8080").await?;

    client.write_all(b"Ini baru nyoba aja...").await?;

    Ok(())
}
