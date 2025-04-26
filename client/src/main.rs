use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use common::*;

#[cfg(target_os = "windows")]
fn get_os() -> OperatingSystems {
    OperatingSystems::Windows
}

#[cfg(target_os = "linux")]
fn get_os() -> OperatingSystems { 
    OperatingSystems::Linux
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = bincode::config::standard();
    let addr = "127.0.0.1:9000";
    let mut stream = TcpStream::connect(addr).await?;
    println!("[*] Connected to {}", addr);
    
    // Send client information
    let client_info = ClientInfo{client_type: ClientType::Client, os: get_os()};
    let encoded = bincode::encode_to_vec(&client_info, config)?;
    stream.write_all(&encoded).await;

    Ok(())
}
