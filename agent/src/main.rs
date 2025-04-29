// Agent
use common::*;
use tcp::*;

#[cfg(feature = "tcp")] 
pub mod tcp;

trait Connection {
    async fn connect(&mut self) -> anyhow::Result<()>;
    async fn send<E: bincode::Encode>(&mut self, data: E) -> anyhow::Result<Message>;
    async fn recieve(&mut self) -> anyhow::Result<Message>;
    
    async fn _send_client_info(&mut self) -> anyhow::Result<()> {
        let client_info = ClientInfo::new(ClientType::Agent, get_os());
        self.send(client_info).await?;
        Ok(())
    }

    fn encode<E: bincode::Encode>(&mut self, data: E) -> Vec<u8> {
       let config = bincode::config::standard();
        bincode::encode_to_vec(data, config).unwrap()
    }

    fn decode(&mut self, data: &[u8]) -> anyhow::Result<Message> {
        let config = bincode::config::standard();
        let (result, _): (Message, usize) = bincode::decode_from_slice(data, config)?; 
        Ok(result)
    }
}

#[tokio::main]
async fn main() {
    let name = env!("AGENT_NAME");
    let agent_type = env!("AGENT_TYPE");
    let addr = env!("ADDR");
    println!("Hello, my name is {}, my type is {} and I need to connect to: {}", name, agent_type, addr);

    let mut conn = TcpConnection::new(addr).await.unwrap();
    let response = conn.send( Message::Echo { payload: Some("hello from agent".to_string()) }).await.unwrap();
    // println!("Response from server: {:?}", response);
}
