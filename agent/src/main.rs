mod client;

use client::pb::commander_client::CommanderClient;
use client::agent_stream_manager;

#[tokio::main]
async fn main() {
    let mut client = CommanderClient::connect("http://[::1]:50051").await.unwrap();
    agent_stream_manager(&mut client).await;
}
