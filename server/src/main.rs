// Server
use tokio::net::TcpListener;

pub mod client;
use client::*;
pub mod listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("Server Listening on port 9000!");

    loop {
        // Accept and create new client
        let (socket, addr) = listener.accept().await?;
        let mut client = Client::new(socket, addr).await?;

        tokio::spawn(async move {
            let _ = client.client_handler().await;
        });
    }
}
