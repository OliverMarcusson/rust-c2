use bincode::{Decode, Encode};

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

#[derive(Debug, Encode, Decode)]
pub enum ClientType {
    Client,
    Agent,
}
