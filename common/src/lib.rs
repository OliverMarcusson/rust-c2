// Common
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode, PartialEq)]
pub enum ServerMessage {
    Success,
    Failure
}

#[derive(Debug, Encode, Decode)]
pub enum OperatorMessage {
    Echo { message: String },
    Listener { action: ListenerAction },
    Disconnected
}

#[derive(Debug, Encode, Decode)]
pub enum ListenerAction {
    Add {
        name: String,
        listener_type: ListenerType,
    },
}

#[derive(Debug, Encode, Decode)]
pub enum ListenerType {
    Tcp { addr: String },
}

#[derive(Debug, Encode, Decode)]
pub enum Message {
    Echo {
        payload: Option<String>,
    },
    Register,
    Task,
    Exit,
    File,
    Generate {
        name: String,
        os: OperatingSystem,
        listener: ListenerType,
    },
    Listener {
        action: ListenerAction,
    },
}

#[derive(Debug, Encode, Decode)]
pub enum OperatingSystem {
    Windows,
    Linux,
}

#[derive(Debug, Encode, Decode)]
pub struct ClientInfo {
    pub client_type: ClientType,
    pub os: OperatingSystem,
}

impl ClientInfo {
    pub fn new(client_type: ClientType, os: OperatingSystem) -> Self {
        ClientInfo { client_type, os }
    }
}

#[derive(Debug, Encode, Decode)]
pub enum ClientType {
    Client,
    Agent,
}

#[cfg(target_os = "windows")]
pub fn get_os() -> OperatingSystem {
    OperatingSystem::Windows
}

#[cfg(target_os = "linux")]
pub fn get_os() -> OperatingSystem {
    OperatingSystem::Linux
}
