use std::sync::{Arc, Mutex};

use crate::connection_map::{ConnectionInfo, ConnectionMap};

type SharedConnectionMapReference = Arc<Mutex<ConnectionMap>>;

#[derive(Debug)]
pub struct Commander {
    connections: SharedConnectionMapReference
}

impl Commander {
    pub fn new()->Self {
        Self{
            connections: Arc::new(Mutex::new(ConnectionMap::new()))
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
}
