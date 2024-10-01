use std::sync::Arc;

use axum::{routing::get, Router};

use crate::commander_state::Commander;

pub struct CommanderAPI {
    bind_address: String,
    commander_state: Arc<Commander>
}

impl CommanderAPI {
    pub fn new(
        http_port: i64,
        commander_state: Arc<Commander>
    ) -> Self {
        Self{
            bind_address: format!("0.0.0.0:{}", http_port),
            commander_state
        }
    }

    pub async fn start(&self) {
        let commander_state_clients = self.commander_state.clone();
        let commander_state_events = self.commander_state.clone();

        let app = Router::new()
            // FIXME: user must provide an id for the desired connection for which the version shall be required!
            .route("/version", get(version_handler))
            .route("/clients", get(|| active_clients_handler(commander_state_clients)))
            .route("/events", get(|| get_events_handler(commander_state_events)));

        let listener = tokio::net::TcpListener::bind(self.bind_address.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
        // TODO: handle server close errors etc...
    }
}

// TODO: move somewhere else this handler, possibly in a Controller struct?
pub async fn version_handler() -> String {
    "Hello, World!".to_string()

    // let response = commander_server.send(Version_Command).await;
}

pub async fn active_clients_handler(state: Arc<Commander>) -> String {
    let connections = state.get_connections_ids();
    let mut response = String::new();
    for connection in connections {
        response.push_str(&format!("{}\n", connection));
    }
    response
}

pub async fn get_events_handler(state: Arc<Commander>) -> String {
    let events = state.get_events();
    let mut response = String::new();
    for event in events {
        response.push_str(&format!("{:?}\n", event));
    }
    response
}
