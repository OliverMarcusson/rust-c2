use std::net::SocketAddr; 
use rand::{rngs::OsRng, TryRngCore};
use tokio::net::TcpStream;

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

// Agent trait
pub trait Agent {
    async fn send(&self, task: AgentTask) -> anyhow::Result<()>;
    async fn recieve(&self) -> anyhow::Result<AgentMessage>;
}

// Tcp Agent
pub struct TcpAgent {
    pub name: String,
    socket: TcpStream,  
    addr: SocketAddr,
}

impl TcpAgent {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        // Generate random hex name 
        let mut rand_bytes = vec![0u8; 4];
        OsRng.try_fill_bytes(&mut rand_bytes);
        let name = hex::encode(rand_bytes);

        TcpAgent { name, socket, addr}
    }
}

impl Agent for TcpAgent {
    async fn send(&self, task: AgentTask) -> anyhow::Result<()> {
        todo!()
    }
    async fn recieve(&self) -> anyhow::Result<AgentMessage> {
        todo!()
    }
}

// Thread that listens for new agents.
// Spawns handlers for new connections.
pub async fn agent_listener() -> anyhow::Result<()> {Ok(())}

// Thread that handles agents.
// Recieves and responds to commands and callbacks.
pub async fn agent_handler<T: Agent>(agent: T) -> anyhow::Result<()> {Ok(())}

