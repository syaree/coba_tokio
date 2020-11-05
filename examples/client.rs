use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let client = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut rd, mut wr) = io::split(client);
    let mut buf = vec![0; 1024];

    tokio::task::spawn(async move {
        wr.write_all(b"percobaan 1\n").await.unwrap();
        wr.write_all(b"percobaan 2").await.unwrap();
    });

    loop {
        let len = rd.read(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..len]);

        if len == 0 { break; }
        println!("Client got: {:?}", message);
    }

    Ok(())
}
