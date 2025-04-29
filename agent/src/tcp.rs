use common::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::Connection;

pub struct TcpConnection {
    addr: &'static str,
    stream: TcpStream
}

impl TcpConnection {
pub async fn new(addr: &'static str) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let mut conn = TcpConnection { addr, stream };
        let _ = conn._send_client_info().await;

        Ok(conn)
    }
}

#[cfg(feature = "tcp")]
impl Connection for TcpConnection {
    async fn connect(&mut self) -> anyhow::Result<()> {
        use tokio::net::TcpStream;

        self.stream = TcpStream::connect(self.addr).await?;
        let client_info = ClientInfo::new(ClientType::Agent, get_os()); 
        self.send(client_info).await?;
        Ok(())
    }

    async fn send<E: bincode::Encode>(&mut self, data: E) -> anyhow::Result<Message> {
        let encoded = self.encode(data);
        let _ = self.stream.write_all(&encoded).await; 
        
        // Reading server response
        self.recieve().await 
    }

    async fn recieve(&mut self) -> anyhow::Result<Message> {
        let mut buf = vec![0u8; 1024];
        let n = self.stream.read(&mut buf).await?;
        self.decode(&buf[..n])
    }
}
