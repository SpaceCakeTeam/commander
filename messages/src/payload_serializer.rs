use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use super::error::Error;

pub fn serialize<T: Serialize>(payload: &T) -> Result<Vec<u8>, Error> {
    let result = serde_json::to_vec(payload);
    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(Error { message: e.to_string() }),
    }
}

pub fn deserialize<T>(payload: &Vec<u8>) -> Result<T, Error> where T: DeserializeOwned {
    if payload.is_empty() {
        return Err(Error { message: "empty payload".to_string() });
    }
    let result = serde_json::from_slice(payload);
    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(Error { message: e.to_string() }),
    }
}

#[cfg(test)]
mod serialize_tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct PayloadTest {
        a: String
    }

    #[test]
    fn test_serialize_ok() {
        let payload = PayloadTest { a: "hello".to_string() };
        let serialized = serialize(&payload).unwrap();
        assert_eq!(serialized, b"{\"a\":\"hello\"}");
    }

    #[test]
    fn test_deserialize_ok() {
        let serialized_payload = b"{\"a\":\"hello\"}".to_vec();
        let deserialized = deserialize::<PayloadTest>(&serialized_payload).unwrap();
        assert_eq!(deserialized, PayloadTest { a: "hello".to_string() });
    }

    #[test]
    fn test_deserialize_empty_payload() {
        let serialized_payload = Vec::new();
        let deserialized_result = deserialize::<PayloadTest>(&serialized_payload);
        assert_eq!(deserialized_result.unwrap_err().message, "empty payload".to_string());
    }
}
