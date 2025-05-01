use std::net::SocketAddr; 

use rand::{rngs::OsRng, TryRngCore};
// Server Agent Logic
use tokio::{net::TcpStream, sync::mpsc};

// Messages that agents send back to the server.
pub enum AgentMessage {
    CheckIn,
    Callback,
    Output,
    File
}

// Tasks that operators send through the server to agents.
pub enum AgentTask {
    Echo,
    Command,
    Exit,
    CheckIn
}

pub trait Agent {
    async fn send() -> anyhow::Result<()>;
    async fn recieve() -> anyhow::Result<()>;
}

pub struct TcpAgent {
    name: String,
    socket: TcpStream,
    addr: SocketAddr
}

impl TcpAgent {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        let mut rand_bytes = vec![0u8; 4];
        OsRng.try_fill_bytes(&mut rand_bytes);
        let name = hex::encode(rand_bytes);
        TcpAgent { name, socket, addr }
    }
}

impl Agent for TcpAgent {
    async fn send() -> anyhow::Result<()> {
        todo!()
    }
    async fn recieve() -> anyhow::Result<()> {
        todo!()
    }
}

// Thread that listens for new clients (operators or agents).
// Spawns handlers for new connections.
pub async fn agent_listener() -> anyhow::Result<()> {Ok(())}

// Thread that handles clients (operators or agents).
// Recieves and responds to commands and callbacks.
// Can send messages to the listener manager to start or stop listeners.
pub async fn agent_handler<T: Agent>(agent: T) -> anyhow::Result<()> {Ok(())}

