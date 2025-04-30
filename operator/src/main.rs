// Operator
use common::*;
use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn handle_command(stream: &mut TcpStream, input: Vec<&str>) -> anyhow::Result<()> {
    let config = bincode::config::standard();
    let command = input[0];

    match command {
        "echo" if input.len() != 2 => println!("Usage: echo <message>"),
        "echo" => {
            let message = Message::Echo {
                payload: Some(input[1].to_string()),
            };
            let encoded = bincode::encode_to_vec(&message, config)?;
            println!("[*] Echoing {}", input[1]);
            let _ = stream.write_all(&encoded).await;
        }

        "listener" => {
            if input.len() != 4 && input.len() != 3 {
                println!("Usage: listener add/remove");
                println!("Add: listener add <name> <type>");
                println!("Types: tcp");
                println!("\nRemove: listener remove <name>");
                return Ok(());
            }

            if input[1] == "add" {
                let name = input[2];
                match input[3] {
                    "tcp" => {
                        let mut addr = String::new();
                        print!("Addr (ip:port): ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut addr)
                            .expect("Failed to read input");

                        let message = Message::Listener {
                            action: ListenerAction::Add {
                                name: name.to_string(),
                                listener_type: ListenerType::Tcp {
                                    addr: addr.trim().to_string(),
                                },
                            },
                        };
                        let encoded = bincode::encode_to_vec(&message, config)?;
                        let _ = stream.write_all(&encoded).await;
                        println!("[*] Tasked server to add listener '{}'", name);
                    }
                    _ => {
                        println!("Error: Invalid listener type.");
                        return Ok(());
                    }
                }
            }
        }

        _ => {
            todo!();
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = bincode::config::standard();
    let addr = "127.0.0.1:9000";
    let mut stream = TcpStream::connect(addr).await?;
    println!("[*] Connected to {}!", addr);

    // Send client information
    println!("[*] Sending client information.");
    let client_info = ClientInfo {
        client_type: ClientType::Client,
        os: get_os(),
    };
    let encoded = bincode::encode_to_vec(&client_info, config)?;
    let _ = stream.write_all(&encoded).await;

    // Client cycle
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(" ").collect();
        let _ = handle_command(&mut stream, input).await;
    }
}
