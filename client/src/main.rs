use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use common::*;
use std::io::{self, Write};

#[cfg(target_os = "windows")]
fn get_os() -> OperatingSystem {
    OperatingSystem::Windows
}

#[cfg(target_os = "linux")]
fn get_os() -> OperatingSystem { 
    OperatingSystem::Linux
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = bincode::config::standard();
    let addr = "127.0.0.1:9000";
    let mut stream = TcpStream::connect(addr).await?;
    println!("[*] Connected to {}!", addr);
    
    // Send client information
    println!("[*] Sending client information.");
    let client_info = ClientInfo{client_type: ClientType::Client, os: get_os()};
    let encoded = bincode::encode_to_vec(&client_info, config)?;
    let _ = stream.write_all(&encoded).await;

    // Client cycle
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(" ").collect();
        let command = input[0];

        match command {
            "echo" if input.len() != 2 => println!("Usage: echo <message>"), 
            "echo" => {
                let message = Message::Echo { payload: Some(input[1].to_string()) };
                let encoded = bincode::encode_to_vec(&message, config)?;
                println!("[*] Echoing {}", input[1]);
                let _ = stream.write_all(&encoded).await;
            },

            "listener" => {
                if input.len() != 2 {
                    println!("Usage: echo <message>");
                }
            },

            _ => {
                todo!();
            }
        }
    }
}
