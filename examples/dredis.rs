use std::io;

use anyhow::Result;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:16379";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (stream, _) = listener.accept().await?;
        info!("Accepted connection from: {}", stream.peer_addr()?);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream).await {
                warn!("Error processing connection: {:?}", e);
            }
            // process_redis_conn(stream).await
        });
    }
}

async fn process_redis_conn(mut stream: TcpStream) -> Result<()> {
    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    warn!("Connection {} closed", stream.peer_addr()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[tokio::test]
    async fn test_unfragilable() {
        tokio::spawn(async move { Ok::<_, anyhow::Error>(say().await?) });
    }

    async fn say() -> Result<()> {
        println!("say");
        Ok(())
    }
}
