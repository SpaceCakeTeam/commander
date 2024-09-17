use std::net::ToSocketAddrs;
use tonic::transport::Server;
use messages;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::CommanderServer {};
    Server::builder()
        .add_service(messages::pb::commander_server::CommanderServer::new(server))
        .serve("[::0]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}
