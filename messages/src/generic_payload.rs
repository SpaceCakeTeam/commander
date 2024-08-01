use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct PayloadTest {
    a: String
}

pub fn serialize(payload: &PayloadTest) -> Vec<u8> {
    let z = serde_json::to_string(payload);
    z
}