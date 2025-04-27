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
        match &self.ltype {
            ListenerType::Tcp { addr } => {
                let listener = TcpListener::bind(addr).await?;
                println!("[*] Started listener '{}'.", &self.name);
                loop {
                    let (socket, addr) = listener.accept().await?;
                    let agent = Client::new(socket, addr).await?;
                    tokio::spawn(async move {
                        let _ = agent_handler(agent).await;
                    });
                }
            }
        }
    }
}

async fn agent_handler(agent: Client) -> anyhow::Result<()> {
    todo!();
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
        let (info, _): (ClientInfo, usize) = bincode::decode_from_slice(&buf[..], config)?;
        let client = Client { client_type: info.client_type, os: info.os, socket, addr };
        println!("[*] New connection: {}", client.addr);
        println!("[*] Client type: {:?}", client.client_type);
        println!("[*] OS: {:?}", client.os);

        Ok(client)
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
            Ok(0) => {
                println!("[*] Client {:?} disconnected.", client.addr);
                return Ok(());
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
                                match &listener_type {
                                    ListenerType::Tcp { addr: _ } => {
                                        let mut new_listener = Listener::new(name, listener_type);
                                        let _ = new_listener.start().await;
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

        tokio::spawn(async move {
            let _ = client_handler(client).await; 
        });
    }
}
