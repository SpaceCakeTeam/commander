use std::collections::{hash_map::Keys, HashMap};

#[derive(Debug,PartialEq)]
pub struct ConnectionInfo {
    pub channel_id: String
    // TODO: add references to tx/rx
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
            channel_id: "some-id".to_string()
        };

        let mut map = ConnectionMap::new();
        map.set(&"test".to_string(), test_info);

        assert_eq!(map.get(&"test".to_string()), Some(&ConnectionInfo{
            channel_id: "some-id".to_string()
        }));
    }


    #[test]
    fn test_get_unknown_key() {
        let test_info= ConnectionInfo{
            channel_id: "some-id".to_string()
        };

        let mut map = ConnectionMap::new();
        map.set(&"test".to_string(), test_info);

        assert_eq!(map.get(&"test2".to_string()), None);
    }
}
