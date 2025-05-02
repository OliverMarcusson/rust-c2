// Main
mod agent;
mod operator;
mod listener;

use std::{collections::HashMap, sync::Arc};

use agent::*;
use common::{ListenerAction, ListenerType};
use listener::Listener;
use operator::*;
use tokio::{net::TcpListener, sync::{mpsc::{self, Receiver}, Mutex}};

pub struct ListenerManager<T: Listener> {
    listeners: HashMap<String, Arc<Mutex<T>>>,
}

impl<T: Listener + Send + 'static> ListenerManager<T> {
    pub fn new() -> Self {
        let listeners: HashMap<String, Arc<Mutex<T>>> = HashMap::new();
        ListenerManager { listeners }
    }
    
    // Adds and starts the listener.
    pub async fn add(&mut self, listener: T) -> anyhow::Result<()> {
        let name = listener.get_name();
        let listener = Arc::new(Mutex::new(listener));
        self.listeners.insert(name.clone(), listener.clone());
        // let listener_ref = self.listeners.get_mut(&name).unwrap(); 
        tokio::spawn(async move {
            listener.lock().await.start().await;
        });
        Ok(())
    }
}

pub struct AgentManager<T: Agent> {
    agents: HashMap<String, T>,
}

impl<T: Agent> AgentManager<T> {
    pub fn new() -> Self {
        let agents: HashMap<String, T> = HashMap::new();
        AgentManager { agents }
    }

    // Adds and starts handling the agent.
    pub async fn add(&mut self, agent: T) -> anyhow::Result<()> {
        todo!()
    }
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

    // tokio::spawn(listener_manager(listener_manager_rx));
    tokio::spawn(operator_listener(listener, listener_manager_tx));

    // Making sure server never exits by waiting for a future forever.
    Ok(std::future::pending().await)
}

