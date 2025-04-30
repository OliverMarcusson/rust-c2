// Improved main 

// Manager thread that spawns and shuts down listseners.
// Based on requests from a message queue.
async fn listener_manager() -> anyhow::Result<()> {Ok(())}

// Thread that listens for new clients (operators or agents).
// Spawns handlers for new connections.
async fn client_listener() -> anyhow::Result<()> {Ok(())}

// Thread that handles clients (operators or agents).
// Recieves and responds to commands and callbacks.
// Can send messages to the listener manager to start or stop listeners.
async fn client_handler() -> anyhow::Result<()> {Ok(())}

// Thread that generates agent binaries from requests in a queue.
// Recieves requests from client handlers.
async fn agent_generator() -> anyhow::Result<()> {Ok(())}

// Manager thread that handles active agent state.
// Gives out agent information when requested from a queue.
// Removes agent state when tasked by message.
async fn agent_manager() -> anyhow::Result<()> {Ok(())}

// Main function
// Spawns manager threads.
#[tokio::main]
async fn main() -> anyhow::Result<()> {Ok(())}

