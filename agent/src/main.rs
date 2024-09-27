mod client;

use std::env;

use messages::pb::commander_client::CommanderClient;
use client::agent_stream_manager;

mod k8scommands;

#[tokio::main]
async fn main() {
    let address = env::var("COMMANDER_URL").unwrap_or("http://[::1]:50051".to_string());
    println!("Connecting to commander at {}", address);
    let mut client = CommanderClient::connect(address).await.unwrap();
    agent_stream_manager(&mut client).await;
}
