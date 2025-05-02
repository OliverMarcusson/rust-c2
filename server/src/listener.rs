use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use crate::AgentManager;
use crate::agent::TcpAgent;

pub trait Listener {
    fn start(&mut self) -> impl std::future::Future<Output = anyhow::Result<()>> + std::marker::Send;
    // async fn start(&mut self) -> anyhow::Result<()> ;
    fn get_name(&self) -> String;
}

pub struct TcpListener {
    name: String,
    listener: tokio::net::TcpListener,
    agent_manager: Arc<Mutex<AgentManager<TcpAgent>>>,
    running: bool
}

impl TcpListener {
    pub async fn new(name: String, addr: SocketAddr, agent_manager: Arc<Mutex<AgentManager<TcpAgent>>>) -> anyhow::Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        // Adds the listener to the manager
        Ok(TcpListener { name, listener, agent_manager, running: false })
    } 
}

impl Listener for TcpListener {
     async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        while self.running {
            let (socket, addr) = self.listener.accept().await?;
            let agent = TcpAgent::new(socket, addr);
            self.agent_manager.lock().await.add(agent);
        }
        Ok(())
    }

     fn get_name(&self) -> String {
         self.name.clone()
     }
}
