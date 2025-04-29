use crate::listener::*;
use common::*;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    client_type: ClientType,
    os: OperatingSystem,
    socket: TcpStream,
    addr: SocketAddr,
}
impl Client {
    pub async fn new(mut socket: TcpStream, addr: SocketAddr) -> anyhow::Result<Client> {
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
        let client = Client {
            client_type: info.client_type,
            os: info.os,
            socket,
            addr,
        };
        println!("[*] New connection: {}", client.addr);
        println!("[*] Client type: {:?}", client.client_type);
        println!("[*] OS: {:?}", client.os);

        Ok(client)
    }

    pub async fn client_handler(mut self) -> anyhow::Result<()> {
        let config = bincode::config::standard();
        loop {
            let mut buf = vec![0u8; 1024];
            let message_len = self.socket.read(&mut buf).await;

            // Error handling client message
            match message_len {
                // Client disconnected
                Ok(0) => {
                    println!("[*] Client {:?} disconnected.", self.addr);
                    return Ok(());
                }

                // Valid client message
                Ok(n) => {
                    println!("[*] Message recieved from {:?}", self.addr);
                    let (message, _len): (Message, usize) =
                        bincode::decode_from_slice(&buf[..n], config)?;

                    match message {
                        Message::Echo { payload: echo } => match echo {
                            Some(echo) => {
                                println!("[*] Echo: {}", echo.trim());
                            }
                            None => {
                                return Err(anyhow::anyhow!("Message not supplied"));
                            }
                        },

                        Message::Listener { action } => match action {
                            ListenerAction::Add {
                                name,
                                listener_type,
                            } => match listener_type {
                                ListenerType::Tcp { addr: _ } => {
                                    println!("[*] Add Tcp Listener");
                                    let mut new_listener = Listener::new(name, listener_type);
                                    let _ = new_listener.start().await;
                                }
                            },
                        },
                        _ => {
                            println!("Unimplemented message: {:?}", message);
                        }
                    }
                }

                // Invalid client message
                Err(e) => {
                    println!("Error while recieving client message: {:?}", e);
                }
            }
        }
    }
}
