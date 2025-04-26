use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum OperatingSystems {
    Windows,
    Linux,
}

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_type: ClientType,
    pub os: OperatingSystems,
}

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum ClientType {
    Client,
    Agent,
}
