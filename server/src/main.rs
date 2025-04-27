use std::net::SocketAddr;

use common::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct Listener {
    name: String,
    ltype: ListenerType
}

impl Listener {
    fn new(name: String, ltype: ListenerType) -> Self {
        Listener { name, ltype }
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        todo!();
        // match self.ltype {
        //     ListenerType::Tcp { name, addr }
        // }
    }
}

#[derive(Debug)]
pub struct Client {
    client_type: ClientType,
    os: OperatingSystem,
    socket: TcpStream,
    addr: SocketAddr,
}
impl Client {
    async fn new(mut socket: TcpStream, addr: SocketAddr) -> anyhow::Result<Client> {
        let config = bincode::config::standard();

        // Recieve client info
        let mut buf = vec![0u8; 1024];
        let n = socket.read(&mut buf).await?;

        // If response length is zero
        if n == 0 {
            return Err(anyhow::anyhow!(
                "Client disconnected before sending initial information."
            ));
        }

        // Decode info from client with bincode
        let (info, len): (ClientInfo, usize) = bincode::decode_from_slice(&buf[..], config)?;
        Ok(Client {
            client_type: info.client_type,
            os: info.os,
            socket,
            addr,
        })
    }
}

async fn client_handler(mut client: Client) -> anyhow::Result<()> {
    let config = bincode::config::standard();
    loop {
        let mut buf = vec![0u8; 1024];
        let message_len = client.socket.read(&mut buf).await; 
        
        // Error handling client message
        match message_len {
            // Client disconnected
            Ok(n) if n == 0 => {
                println!("[*] Client {:?} disconnected.", client.addr);
            }

            // Valid client message
            Ok(n) => {
                println!("[*] Message recieved from {:?}", client.addr);
                let (message, _len): (Message, usize)= bincode::decode_from_slice(&buf[..n], config)?;
                
                match message {
                    Message::Echo { payload: echo} => {
                        match echo {
                            Some(echo) => {
                                println!("[*] Echo: {}", echo.trim());

                            }
                            None => {
                                return Err(anyhow::anyhow!("Message not supplied"));
                            }
                        }                    
                    },

                    Message::Listener { action } => {
                        match action {
                            ListenerAction::Add { name, listener_type } => {
                                match listener_type {
                                    ListenerType::Tcp { name: listener_name, addr } => {

                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        unimplemented!();
                    }
                }
            },

            // Invalid client message
            Err(e) => {
                println!("Error while recieving client message: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    println!("Server Listening on port 9000!");

    loop {
        // Accept and create new client
        let (socket, addr) = listener.accept().await?;
        let client = Client::new(socket, addr).await?;
        println!("[*] New connection: {}", client.addr);
        println!("[*] Client type: {:?}", client.client_type);
        println!("[*] OS: {:?}", client.os);

        tokio::spawn(async move {
            let _ = client_handler(client).await; 
        });
    }
}
