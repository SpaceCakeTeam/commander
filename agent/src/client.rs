pub mod pb {
  tonic::include_proto!("messages");
}

use std::time::Duration;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;

use pb::{commander_client::CommanderClient, Message};

fn message_requests_iter() -> impl Stream<Item = Message> {
  tokio_stream::iter(1..usize::MAX).map(|i| Message {
      name: format!("msg {:02}", i),
      timestamp: "".to_string(),
      payload: vec![],
  })
}

pub async fn bidirectional_streaming_echo_throttle(client: &mut CommanderClient<Channel>, dur: Duration) {
  let in_stream = message_requests_iter().throttle(dur);

  let response = client
      .channel(in_stream)
      .await
      .unwrap();

  let mut resp_stream = response.into_inner();

  while let Some(received) = resp_stream.next().await {
      let received = received.unwrap();
      println!("\treceived message: `{:#?}`", received);
  }
}
