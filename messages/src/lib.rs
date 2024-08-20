pub mod pb {
  tonic::include_proto!("messages");
}

pub mod payload_serializer;
pub mod error;
pub mod messages;

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Sender;
use tonic::Status;
use pb::Message;

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

#[cfg(test)]
mod messages_tests {
  use tokio::sync::mpsc;
  use super::*;


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
}
