use std::{os::macos::raw::stat, sync::Arc};

use axum::{extract::Path, routing::get, Router};
use messages::{definitions::K8S_GET_VERSION_COMMAND, pb::Message, timenow};

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
        let commander_state_version = self.commander_state.clone();
        let commander_state_clients = self.commander_state.clone();
        let commander_state_events = self.commander_state.clone();

        let app = Router::new()
        // FIXME: user must provide an id for the desired connection for which the version shall be required!
        .route("/channels/:channel_id/version", get(|Path(channel_id): Path<String>| version_handler(commander_state_version, channel_id)))
        .route("/clients", get(|| active_clients_handler(commander_state_clients)))
        .route("/events", get(|| get_events_handler(commander_state_events)));

        let listener = tokio::net::TcpListener::bind(self.bind_address.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
        // TODO: handle server close errors etc...
    }
}

// TODO: move somewhere else this handler, possibly in a Controller struct?
pub async fn version_handler(state: Arc<Commander>, ch_id: String) -> String {
    let connection = state.get_connection(ch_id);
    if connection.is_some() {
        let message = Message{
            name: K8S_GET_VERSION_COMMAND.to_string(),
            timestamp: timenow(),
            payload: vec![],
        };
        connection.as_ref().clone().unwrap().send(message).await;
        format!("Version command sent\n")
    } else {
        format!("Connection not found\n")
    }
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
