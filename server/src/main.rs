// Main
mod agent;
mod operator;

use agent::*;
use operator::*;
use tokio::net::TcpListener;

// Manager thread that spawns and shuts down listseners.
// Based on requests from a message queue.
async fn listener_manager() -> anyhow::Result<()> {Ok(())}

// Thread that generates agent binaries from requests in a queue.
// Recieves requests from client handlers.
async fn agent_generator() -> anyhow::Result<()> {Ok(())}

// Manager thread that handles active agent state.
// Gives out agent information when requested from a queue.
// Removes agent state when tasked by message.
async fn agent_manager() -> anyhow::Result<()> {Ok(())}

// Main function
// Spawns manager threads, creates message queues.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:1337";
    let listener = TcpListener::bind(addr).await?;
    println!("[*] Server running at {addr}.");

    tokio::spawn(operator_listener(listener));

    // Making sure server never exits by waiting for a future forever.
    Ok(std::future::pending().await)
}

