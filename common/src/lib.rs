use serde::{Deserialize, Serialize};
use bincode::{Encode, Decode};

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum ListenerAction {
    Add { name: String, listener_type: ListenerType }
}

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum ListenerType {
    Tcp { name: String, addr: String }
}

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum Message {
    Echo { payload: Option<String> },
    Register,
    Task,
    Exit,
    File,
    Generate { name: String, os: OperatingSystem, listener: ListenerType },
    Listener { action: ListenerAction }
}


#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum OperatingSystem {
    Windows,
    Linux,
}

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_type: ClientType,
    pub os: OperatingSystem,
}

#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub enum ClientType {
    Client,
    Agent,
}
