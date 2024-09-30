use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub struct ConnectionInfo {
    ch_id: String
}

#[derive(Debug)]
pub struct ConnectionMap<'a> {
    hashmap: HashMap<String, &'a ConnectionInfo>
}

impl ConnectionMap<'_> {
    pub fn new()->Self{Self{
        hashmap: HashMap::new(),
    }}

    pub fn set(&mut self, key: String, info: &ConnectionInfo) {
        self.hashmap.insert(key, info);
    }

    // pub fn get(self, key: String)->Option<ConnectionInfo> {
    //     self.hashmap.get(&key)
    // }
}

#[cfg(test)]
mod connection_map_tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let test_info= ConnectionInfo{
            ch_id: "some-id".to_string()
        };
    
        let mut map = ConnectionMap::new();        
        map.set("test".to_string(), &test_info);

        assert_eq!(map.hashmap.get("test"), Some(test_info));

        // let returned_option = map.get("test".to_string());
        // assert_eq!(returned_option.unwrap(), test_info);
    }
}
