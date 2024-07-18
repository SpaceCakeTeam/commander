pub mod pb {
  tonic::include_proto!("messages");
}

use std::{thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

use tokio::sync::mpsc::{self, Sender};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;

use pb::{commander_client::CommanderClient, Message};

fn timenow() -> u128 {
  return SystemTime::now()
  .duration_since(UNIX_EPOCH)
  .unwrap()
  .as_millis()
  .try_into()
  .unwrap()
}

pub async fn agent_stream_manager(client: &mut CommanderClient<Channel>) {
  println!("CLI START {:#?}", timenow());

  let (mut tx, rx) = mpsc::channel(128);

  let ch = ReceiverStream::new(rx);
  let response: tonic::Response<tonic::Streaming<Message>> = client
    .channel(ch)
    .await
    .unwrap();

  let mut resp_stream = response.into_inner();

  while let Some(received) = resp_stream.next().await {
    let received = received.unwrap();
    println!("received {:#?}", received);
    send_version(&mut tx).await;
  }

  // FIXME: sleep blocks the process and allows commander to have time processing the message,
  // without this everything crashes. We have to find a way to keep the agent running and keep
  // the connection alive!
  sleep(Duration::from_secs(5))
}

async fn send_version(str: &mut Sender<Message>) {
 let _ =  str.send(Message{
    name: "handshake".to_string(),
    timestamp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis()
      .try_into()
      .unwrap(),
    payload: Vec::new()
  }).await;
}

#[cfg(test)]
mod client_tests {
    use super::*;

    #[tokio::test]
    async fn test_send_version() {
        let (mut tx, mut rx) = mpsc::channel(1);
        send_version(&mut tx).await;
        let actual = rx.recv().await;
        assert_eq!("handshake".to_string(), actual.unwrap().name);
    }
}
