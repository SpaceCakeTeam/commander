pub mod pb {
  tonic::include_proto!("messages");
}

use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::mpsc::{self, Sender};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;

use pb::{commander_client::CommanderClient, Message};

fn timenow() -> u64 {
  return SystemTime::now()
  .duration_since(UNIX_EPOCH)
  .unwrap()
  .as_millis()
  .try_into()
  .unwrap()
}

pub async fn agent_stream_manager(client: &mut CommanderClient<Channel>) {
  println!("agent started at {:#?}", timenow());

  let (mut tx, rx) = mpsc::channel(128);

  let ch = ReceiverStream::new(rx);
  let response: tonic::Response<tonic::Streaming<Message>> = client
    .channel(ch)
    .await
    .unwrap();

  let mut resp_stream = response.into_inner();
  loop {
    match resp_stream.next().await {
      Some(received) => {
        let received = received.unwrap();
        println!("received message {:#?}", received);

        let resp = get_response_message(received);
        send_message(&mut tx, resp).await;

        println!("sent message response {:#?}", timenow());
      },
      None => {
        println!("Received None from stream :( at {:#?}", timenow());
        break;
      }
    }
  }

  println!("closing client!");
}

fn get_response_message(received_message: Message) -> Message {
  match received_message.name.as_str() {
    "handshake" => build_version_message(),
    _ => Message { name: "err".to_string(), timestamp: timenow(), payload: Vec::new() },
  }
}

async fn send_message(str: &mut Sender<Message>, message: Message) {
  let _ = str.send(message).await;
}

fn build_version_message() -> Message {
  Message { name: "handshake_response".to_string(), timestamp: timenow(), payload: Vec::new() }
}

#[cfg(test)]
mod client_tests {
  use super::*;

  #[tokio::test]
  async fn test_send_message() {
    let (mut tx, mut rx) = mpsc::channel(1);
    let msg = Message{
      name: "handshake".to_string(),
      timestamp: timenow(),
      payload: Vec::new(),
    };
    send_message(&mut tx, msg).await;
    let actual = rx.recv().await;
    assert_eq!("handshake".to_string(), actual.unwrap().name);
  }

  #[test]
  fn test_get_response_message() {
    let msg = Message{
      name: "handshake".to_string(),
      timestamp: timenow(),
      payload: Vec::new(),
    };

    let recv: Message = get_response_message(msg);
    assert_eq!("handshake_response", recv.name)
  }
}
