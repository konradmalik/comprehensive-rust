use std::io::Error;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(socket: &mut TcpStream) -> Result<(), Error> {
    socket.write_all(b"Who are you?\n").await?;

    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await?;
    let reply = {
        let name = std::str::from_utf8(&buf[..n]).unwrap().trim();
        format!("Thanks for dialing in, {name}!\n")
    };

    socket.write_all(reply.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    println!("listening on port 6142");

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("connection from {addr:?}");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(&mut socket).await {
                println!("socket error: {}", e);
                return;
            };
        });
    }
}
