use std::{net::ToSocketAddrs, sync::Arc};
use commander_state::Commander;
use http_server::CommanderAPI;
use tonic::transport::Server;
use messages;

mod server;
mod http_server;
mod connection_map;
mod commander_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Commander::new();
    let state_ref = Arc::new(state);
    let state_ref_server_http = state_ref.clone();
    let state_ref_server_grpc = state_ref.clone();
    tokio::spawn(async move {
        let http_server_port = 8080;
        println!("Starting HTTP server on port {}", http_server_port);
        CommanderAPI::new(http_server_port, state_ref_server_http).start().await;
    });

    let grpc_server_port = 50051;
    println!("Starting gRPC server on port {}", grpc_server_port);
    let server = server::CommanderServer::new(state_ref_server_grpc);
    Server::builder()
        .add_service(messages::pb::commander_server::CommanderServer::new(server))
        .serve(format!("[::0]:{}", grpc_server_port).to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
