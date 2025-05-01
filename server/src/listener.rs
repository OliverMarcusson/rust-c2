use std::net::SocketAddr;
use crate::agent::{agent_handler, Agent, TcpAgent};

pub trait Listener {
    async fn start(&mut self) -> anyhow::Result<()>;
    async fn stop(&self) -> anyhow::Result<()>;
}

pub struct TcpListener {
    listener: tokio::net::TcpListener,
    running: bool
}

impl TcpListener {
    pub async fn new(addr: SocketAddr) -> anyhow::Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        Ok(TcpListener { listener, running: false })
    }

    pub async fn accept(&mut self) -> anyhow::Result<()> {
        while self.running {
            let (socket, addr) = self.listener.accept().await?;
            let agent = TcpAgent::new(socket, addr);
            tokio::spawn(agent_handler(agent));
        }
        Ok(())
    }
}

impl Listener for TcpListener {
    async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        // tokio::spawn(self.accept());
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        todo!()
    }
}
