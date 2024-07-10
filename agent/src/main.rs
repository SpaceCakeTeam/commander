mod client;

use client::pb::commander_client::CommanderClient;
use std::time::Duration;
use client::bidirectional_streaming_echo_throttle;

#[tokio::main]
async fn main() {
    let mut client = CommanderClient::connect("http://[::1]:50051").await.unwrap();
    bidirectional_streaming_echo_throttle(&mut client, Duration::from_secs(2)).await;
}
