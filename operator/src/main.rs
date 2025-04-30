// Operator
use common::*;
use std::future;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Server {
    socket: TcpStream,
    addr: SocketAddr
}

impl Server {
    pub async fn connect(name: String, addr: &str) -> anyhow::Result<Self> {
        let mut socket = TcpStream::connect(addr).await?;
        let len = name.len() as u64;
        let _ = socket.write_u64(len).await;
        let _ = socket.write(name.as_bytes()).await;
        Ok( Server { socket, addr: SocketAddr::from_str(addr)? } ) 
    }

    pub async fn recieve(&mut self) -> anyhow::Result<ServerMessage> {
        let len = self.socket.read_u64().await?.try_into().unwrap();
        let mut buf = vec![0u8; len];
        let _ = self.socket.read_exact(&mut buf).await?;
        let (message, _): (ServerMessage, usize) = bincode::decode_from_slice(&buf, bincode::config::standard())?;
        Ok(message)
    }

    pub async fn send(&mut self, message: OperatorMessage) -> anyhow::Result<()> {
        let encoded = bincode::encode_to_vec(&message, bincode::config::standard())?;
        let len = encoded.len() as u64;
        let _ = self.socket.write_u64(len).await;
        let _ = self.socket.write_all(&encoded).await;
        Ok(())
    }

    pub async fn get_response(&mut self, success_message: &str, failure_message: &str) {
        match self.recieve().await {
            Ok(ServerMessage::Success) => println!("{success_message}"),
            Ok(ServerMessage::Failure) => println!("{failure_message}"),
            Err(_) => println!("Failed to get response from server.")
        }
    }
}

fn read_command() -> anyhow::Result<Vec<String>> {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let command = shell_words::split(&input)?;
    Ok(command)
}

enum AddListenerMessage {}
impl AddListenerMessage {
    fn tcp(name: String) -> anyhow::Result<OperatorMessage> {
        let addr = read_line_string("Listener addr (ip:port): ")?;
        Ok(OperatorMessage::Listener { action: ListenerAction::Add { name, listener_type: ListenerType::Tcp { addr } } })
    } 
}

enum Usage {}
impl Usage {
    fn echo() {println!("Usage: echo <message>");}
    fn listener() {
        println!("Usage: listener <action> [args]");
        println!("Actions:");
        println!("  - add:\n    - listener add <name> <type>\n    - types:\n      - tcp");
        println!("  - remove:\n    - listener remove <name>");
    }
}

async fn operator_cycle(mut server: Server) -> anyhow::Result<()> {
    loop {
       let command = read_command()?;
       match command[0].to_lowercase().as_str() {
           "echo" => {
                if command.len() < 2 {Usage::echo();}
                println!("[*] Echoing '{}'", &command[1]);
                server.send(OperatorMessage::Echo { message: command[1].clone() }).await?;
                
                server.get_response(
                    "[] Server successfully echoed message!",
                    "[] Server failed to echo message."
                ).await;

                // let response = server.recieve().await?;
                // if response == ServerMessage::Success {println!("[*] Server successfully echoed message!");}
           },
           "listener" => {
               if command.len() < 2 {Usage::listener(); continue;}
               if command[1].as_str() == "add" && command.len() < 4 {Usage::listener(); continue;}
               if command[1].as_str() == "remove" && command.len() < 3 {Usage::listener(); continue;}
               if command[1].as_str() == "add" {
                   match AddListenerMessage::tcp(command[2].clone()) {
                       Ok(message) => {
                            println!("[*] Tasked server to add listener '{}'", command[2]);
                            server.send(message).await?;
                            server.get_response(
                                format!("[] Server successfully started listener '{}'", command[2]).as_str(),
                                format!("[] Server failed to start listener '{}'", command[2]).as_str(),

                            ).await;
                       }
                       Err(e) => println!("[!] Failed to create listener message: {}", e)
                   }
               }
           },
           unimplemented => println!("Unimplemented command: {unimplemented}")
       } 
    }
}

fn read_line_string(prompt: &str) -> anyhow::Result<String> {
    let mut input = String::new();
    print!("{prompt}");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err(anyhow::anyhow!("Failed to read line."))
    }

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let name = read_line_string("Operator name: ")?; 
    let addr = "127.0.0.1:1337";
    println!("[*] Connecting to server @ {addr}");
    
    match Server::connect(name, addr).await {
        Ok(server) => {
            println!("[*] Successfully connected to server!");
            tokio::spawn(operator_cycle(server));
        },
        Err(err) => println!("Failed to connect to server @ {addr}: {:?}", err)
    }

    // Keep program from exiting.
    Ok(future::pending().await)
}
