use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;

use messages::{pb::{commander_client::CommanderClient, Message}, timenow, send2server};

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
        match resp {
          Some(message_to_send) => send2server(&mut tx, message_to_send).await,
          _ => (),
        }

        println!("processed message {:#?}", timenow());
      },
      None => {
        println!("Received None from stream :( at {:#?}", timenow());
        break;
      }
    }
  }

  println!("closing client!");
}

fn get_response_message(received_message: Message) -> Option<Message> {
  match received_message.name.as_str() {
    "handshake" => Some(build_version_message()),
    _ => None,
  }
}


fn build_version_message() -> Message {
  Message { name: "handshake_response".to_string(), timestamp: timenow(), payload:  Some("version 123"), }
}

#[cfg(test)]
mod client_tests {
  use super::*;

  #[test]
  fn test_get_response_message() {
    let msg = Message{
      name: "handshake".to_string(),
      timestamp: timenow(),
      payload: Some("ciao"),
    };

    let recv = get_response_message(msg);
    assert_eq!("handshake_response", recv.unwrap().name);
    assert_eq!("ciao", recv.unwrap().payload.unwrap());
  }
}
