mod client;

use std::env;

use messages::pb::commander_client::CommanderClient;
use client::agent_stream_manager;
use tokio::signal::{self, unix::{signal as unix_signal, SignalKind}};
use tokio_util::sync::CancellationToken;

mod k8scommands;

#[tokio::main]
async fn main() {
    let address = env::var("COMMANDER_URL").unwrap_or("http://[::1]:50051".to_string());

    let token = CancellationToken::new();

    let agent_client_cancellation_token = token.clone();
    tokio::spawn(async move {
        println!("Connecting to commander at {}", address);
        let mut client = CommanderClient::connect(address).await.unwrap();
        agent_stream_manager(&mut client, agent_client_cancellation_token).await;
    });
    tokio::select! {
        // TODO: Wait for the SIGTERM signal
        // _ = Future::new(unix_signal(SignalKind::terminate())) => {
        //     println!("Received SIGTERM, exiting");
        //     token.cancel();
        // }
        _ = signal::ctrl_c() => {
            println!("Received SIGTERM, exiting");
            token.cancel();
        }
    }
}
