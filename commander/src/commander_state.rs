use std::sync::{Arc, Mutex};

use messages::pb::Message;

use crate::connection_map::{ConnectionInfo, ConnectionMap};

type SharedConnectionMapReference = Arc<Mutex<ConnectionMap>>;

#[derive(Clone, Debug)]
pub struct Event {
    pub message: Message,
    pub channel_id: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Commander {
    connections: SharedConnectionMapReference,
    events: Arc<Mutex<Vec<Event>>>,
}

impl Commander {
    pub fn new()->Self {
        Self{
            connections: Arc::new(Mutex::new(ConnectionMap::new())),
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_connections_ids(&self) -> Vec<String> {
        self.connections.lock().unwrap().keys().cloned().collect()
    }

    pub fn register(&self, key: &String, info: ConnectionInfo) {
        self.connections.lock().unwrap().set(key, info);
      }

    pub fn unregister(&self, key: &String) {
        self.connections.lock().unwrap().rem(key);
    }

    pub fn collect(&self, event: &Event) {
        self.events.lock().unwrap().push(event.clone());
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.events.lock().unwrap().clone()
    }
}
