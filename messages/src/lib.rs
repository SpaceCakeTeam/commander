pub mod pb {
  tonic::include_proto!("messages");
}

pub mod payload_serializer;
pub mod error;
pub mod definitions;

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc::Sender;
use tonic::Status;
use pb::Message;
use error::Error;
use payload_serializer::{deserialize, serialize};

pub fn timenow() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .try_into()
    .unwrap()
}

pub async fn send2server(str: &mut Sender<Message>, message: Message) {
  let _ = str.send(message).await;
}

pub async fn send2client(str: &mut Sender<Result<Message, Status>>, message: Message) {
  let _ = str.send(Ok(message)).await;
}

impl Message {
  pub fn new<T: Serialize>(name: &str, payload: &T) -> Result<Message, Error> {
    let serialized =  serialize(payload)?;
    Ok(Message{
      name: name.to_string(), 
      timestamp: timenow(),
      payload: serialized,
    })
  }

  pub fn get_payload<T>(&self) -> Result<T, Error> where T: DeserializeOwned {
    deserialize(&self.payload)
  }
}

pub fn build_message_or_print_error<T: Serialize>(name: &str, payload: &T) -> Message {
  Message::new(name, payload)
      .map_err(|e| 
          println!("|{}| failed message {} serialization {:#?}", timenow(), name, e.message)
      )
      .ok()
      .unwrap()
}

#[cfg(test)]
mod messages_tests {
  use tokio::sync::mpsc;
  use serde::Deserialize;
  use super::*;

  #[derive(Serialize, Debug, Deserialize, PartialEq)]
  struct PayloadTest {
    a: String
  }

  #[tokio::test]
  async fn test_send2server() {
    let (mut tx, mut rx) = mpsc::channel(1);
    let msg = Message{
      name: "handshake".to_string(),
      timestamp: timenow(),
      payload: Vec::new(),
    };

    send2server(&mut tx, msg).await;
    let actual = rx.recv().await;
    assert_eq!("handshake".to_string(), actual.unwrap().name);
  }

  #[test]
  fn test_message_builder() {
    let message_name = "my-message";
    let message = Message::new(message_name, &PayloadTest{a: "hello".to_string()});
    let expected_vector = b"{\"a\":\"hello\"}";
    assert_eq!(message.unwrap(), Message{
      name: "my-message".to_string(),
      timestamp: timenow(),
      payload: expected_vector.to_vec(),
    });
  }

  #[test]
  fn test_get_payload() {
    let message_name = "my-message";
    let raw_payload_message =  "hello".to_string();
    let raw_payload = PayloadTest{a: raw_payload_message};
    let msg = Message::new(message_name, &raw_payload);
    let deserialized_payload = msg.unwrap().get_payload::<PayloadTest>();
    assert_eq!(deserialized_payload.unwrap(), raw_payload);
  }
}
