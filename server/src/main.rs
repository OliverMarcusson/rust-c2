use std::net::SocketAddr;

use common::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Client {
    client_type: ClientType,
    os: OperatingSystems,
    socket: TcpStream,
    addr: SocketAddr,
}
impl Client {
    async fn new(mut socket: TcpStream, addr: SocketAddr) -> anyhow::Result<Client> {
        let mut buf = vec![0u8; 1024];
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            return Err(anyhow::anyhow!(
                "Client disconnected before sending initial information."
            ));
        }

        let info: ClientInfo = bincode::deserialize(&buf[..n])?;
        Ok(Client {
            client_type: info.client_type,
            os: info.os,
            socket,
            addr,
        })
    }
}

// async fn client_handler(client: Client) -> anyhow::Result<()> {
//     let mut buf = vec![0u8; 1024];
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("Server Listening on port 9000!");

    loop {
        let (socket, addr) = listener.accept().await?;
        let mut client = Client::new(socket, addr).await?;
        println!("New connection: {}!", client.addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            match client.socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => {
                    let s = std::str::from_utf8(&buf[..n - 1])
                        .expect("Failed to convert bytes slice to str.");
                    println!("Recieved {:?}", s)
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        });
    }
}
