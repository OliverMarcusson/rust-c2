// Server Agent Logic
use tokio::sync::mpsc;

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

// Generic trait for all listener types.
// Specifies how to handle connecting agents.
pub trait Listener {
    async fn start(queue: mpsc::Receiver<AgentMessage>) -> anyhow::Result<()> {Ok(())}
}

// Listener for agents to connect through TCP.
pub struct TcpListener {}
impl Listener for TcpListener {}

// Listener for agents to connect through HTTP. 
pub struct HttpListener {}
impl Listener for HttpListener {}


// Thread that listens for new clients (operators or agents).
// Spawns handlers for new connections.
pub async fn agent_listener() -> anyhow::Result<()> {Ok(())}

// Thread that handles clients (operators or agents).
// Recieves and responds to commands and callbacks.
// Can send messages to the listener manager to start or stop listeners.
pub async fn agent_handler() -> anyhow::Result<()> {Ok(())}

