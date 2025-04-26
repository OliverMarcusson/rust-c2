use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Debug, Serialize, Deserialize)]
pub enum OperatingSystems {
    Windows,
    Linux,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_type: ClientType,
    pub os: OperatingSystems,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientType {
    Client,
    Agent,
}
