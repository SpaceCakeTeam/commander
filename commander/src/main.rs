use std::net::ToSocketAddrs;
use http_server::CommanderAPI;
use tonic::transport::Server;
use messages;

mod server;
mod http_server;
mod connection_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let http_server_port = 8080;
        println!("Starting HTTP server on port {}", http_server_port);
        CommanderAPI::new(http_server_port).start().await;
    });

    let grpc_server_port = 50051;
    println!("Starting gRPC server on port {}", grpc_server_port);
    let server = server::CommanderServer {};
    Server::builder()
        .add_service(messages::pb::commander_server::CommanderServer::new(server))
        .serve(format!("[::0]:{}", grpc_server_port).to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
