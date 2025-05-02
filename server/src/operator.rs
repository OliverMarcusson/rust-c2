use std::net::SocketAddr;
use common::{ListenerAction, OperatorMessage, ServerMessage};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::mpsc::Sender};

pub struct Operator {
    name: String,
    socket: TcpStream,
    addr: SocketAddr,
    listener_manager_tx: Sender<ListenerAction>
}

impl Operator {
    pub async fn accept(listener: &mut TcpListener, listener_manager_tx: Sender<ListenerAction>) -> anyhow::Result<Self> {
        let (mut socket, addr) = listener.accept().await?;
        match socket.read_u64().await {
            Ok(0) => {
                println!("Couldn't accept operator, it disconnected???");
                Err(anyhow::anyhow!("Couldn't accept operator, it disconnected???"))
            },
            Ok(len) => {
                let mut buf = vec![0u8; len as usize];
                let _ = socket.read_exact(&mut buf).await;
                let name = String::from_utf8(buf)?;
                Ok(Operator { name, socket, addr, listener_manager_tx })
            },
            Err(e) => {
                Err(anyhow::anyhow!("Couldn't recieve data from operator: {}", e))
            }
        }
    }

    pub async fn recieve(&mut self) -> anyhow::Result<OperatorMessage> {
        let len = self.socket.read_u64().await;
        match len {
            Ok(0) => {
                Ok(OperatorMessage::Disconnected)
            },
            Ok(len) => {
                let mut buf = vec![0u8; len.try_into().unwrap()];
                let _ = self.socket.read_exact(&mut buf).await?;
                let (message, _): (OperatorMessage, usize) = bincode::decode_from_slice(&buf, bincode::config::standard())?;
                Ok(message)
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {return Ok(OperatorMessage::Disconnected);}
                println!("Error while recieving from operator: {:?}", e);
                Err(e.into())
            }
        }
    }

    pub async fn send(&mut self, message: ServerMessage) -> anyhow::Result<()> {
        let encoded = bincode::encode_to_vec(&message, bincode::config::standard())?;
        let len = encoded.len() as u64;
        let _ = self.socket.write_u64(len).await;
        let _ = self.socket.write_all(&encoded).await;
        Ok(())
    }
}


// Thread that listens for new clients (operators or agents).
// Spawns handlers for new connections.
pub async fn operator_listener(mut listener: TcpListener, listener_manager_tx: Sender<ListenerAction>) -> anyhow::Result<()> {
    loop {
        let operator = Operator::accept(&mut listener, listener_manager_tx.clone()).await?;
        println!("[*] Operator {}@{} connected.", &operator.name, &operator.addr);
        
        tokio::spawn(operator_handler(operator));
    }
}

// Thread that handles clients (operators or agents).
// Recieves and responds to commands and callbacks.
// Can send messages to the listener manager to start or stop listeners.
pub async fn operator_handler(mut operator: Operator) -> anyhow::Result<()> {
    loop {
        match operator.recieve().await { 
            Ok(message) => {
                println!("[*] From {}@{}:", operator.name, operator.addr);
                match message {
                    OperatorMessage::Echo { message } => {
                        println!("Echo: {message}");
                        operator.send(ServerMessage::Success).await?;
                    },
                    OperatorMessage::Disconnected => {
                        println!("[*] Operator {}@{} disconnected.", &operator.name, &operator.addr);
                        return Ok(());
                    },
                    OperatorMessage::Listener { action } => {
                        operator.listener_manager_tx.send(action).await;
                    }, 
                    // unimplemented => {
                    //     println!("Unimplemented message: {:?}", unimplemented);
                    //     operator.send(ServerMessage::Failure).await?;
                    // }
                }
            },
            Err(err) => {println!("Failed to recieve message {:?}", err); return Ok(());}
        }
    }
}
