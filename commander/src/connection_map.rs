use std::collections::{hash_map::Keys, HashMap};

use messages::pb::Message;
use tokio::sync::mpsc::Sender;
use tonic::Status;

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub channel_id: String,
    tx: Sender<Result<Message, Status>>,
}

impl PartialEq for ConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl ConnectionInfo {
    pub fn new(channel_id: String, tx: Sender<Result<Message, Status>>) -> Self {
        Self {
            channel_id,
            tx,
        }
    }

    pub async fn send(&self, message: Message) {
        let _ = self.tx.send(Ok(message)).await;
    }
}

#[derive(Debug)]
pub struct ConnectionMap {
    hashmap: HashMap<String, ConnectionInfo>
}

impl ConnectionMap {
    pub fn new()->Self{Self{
        hashmap: HashMap::new(),
    }}

    pub fn set(&mut self, key: &String, info: ConnectionInfo) {
        self.hashmap.insert(key.clone(), info);
    }

    pub fn get(&self, key: &String) -> Option<&ConnectionInfo> {
        self.hashmap.get(key)
    }

    pub fn rem(&mut self, key: &String) {
        self.hashmap.remove(key);
    }

    pub fn keys(&self) -> Keys<String, ConnectionInfo> {
        self.hashmap.keys()
    }
}

#[cfg(test)]
mod connection_map_tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let test_info= ConnectionInfo{
            channel_id: "some-id".to_string(),
            tx: tokio::sync::mpsc::channel(1).0,
        };

        let mut map = ConnectionMap::new();
        map.set(&"test".to_string(), test_info);

        assert_eq!(map.get(&"test".to_string()), Some(&ConnectionInfo{
            channel_id: "some-id".to_string(),
            tx: tokio::sync::mpsc::channel(1).0,
        }));
    }

    #[test]
    fn test_get_unknown_key() {
        let test_info= ConnectionInfo{
            channel_id: "some-id".to_string(),
            tx: tokio::sync::mpsc::channel(1).0,
        };

        let mut map = ConnectionMap::new();
        map.set(&"test".to_string(), test_info);

        assert_eq!(map.get(&"test2".to_string()), None);
    }
}
