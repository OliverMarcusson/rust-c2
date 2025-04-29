use crate::client::Client;
use common::*;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Listener {
    name: String,
    ltype: ListenerType,
}

impl Listener {
    pub fn new(name: String, ltype: ListenerType) -> Self {
        Listener { name, ltype }
    }

    pub async fn start(mut self) -> anyhow::Result<()> {
        match &self.ltype {
            ListenerType::Tcp { addr } => {
                let listener = TcpListener::bind(addr).await?;
                println!("[*] Started listener '{}'.", &self.name);
                tokio::spawn(async move {
                    loop {
                        let (socket, addr) = listener.accept().await.unwrap();
                        let mut agent = Client::new(socket, addr).await.unwrap();
                        tokio::spawn(async move {
                            let _ = agent.client_handler().await;
                        });
                    }
                });
            }
        }
        Ok(())
    }
}
