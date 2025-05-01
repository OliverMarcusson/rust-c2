// Main
mod agent;
mod operator;
mod listener;

use std::collections::HashMap;

use agent::*;
use common::{ListenerAction, ListenerType};
use operator::*;
use tokio::{net::TcpListener, sync::mpsc::{self, Receiver}};

// Manager thread that spawns and shuts down listseners.
// Based on requests from a message queue.
async fn listener_manager(reciever: Receiver<ListenerAction>) -> anyhow::Result<()> {
    let listeners: HashMap<String, Box<dyn Listener>> = HashMap::new();
    println!("[Listener Manager] Running.");
    while let Some(action) = reciever.recv().await {
        match action {
            ListenerAction::Add { name, listener_type } => {
                match listener_type {
                    ListenerType::Tcp { addr } => todo!()
                }
            }
        }
    }
    Ok(())
}

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
    let (listener_manager_tx, listener_manager_rx) = mpsc::channel::<ListenerAction>(100);
    println!("[*] Server running at {addr}.");

    tokio::spawn(listener_manager(listener_manager_rx));
    tokio::spawn(operator_listener(listener, listener_manager_tx));

    // Making sure server never exits by waiting for a future forever.
    Ok(std::future::pending().await)
}

