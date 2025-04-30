// Thread that listens for new clients (operators or agents).
// Spawns handlers for new connections.
pub async fn operator_listener() -> anyhow::Result<()> {Ok(())}

// Thread that handles clients (operators or agents).
// Recieves and responds to commands and callbacks.
// Can send messages to the listener manager to start or stop listeners.
pub async fn operator_handler() -> anyhow::Result<()> {Ok(())}
