pub mod pb {
    tonic::include_proto!("messages");
}

use pb::Message;
use tokio::sync::mpsc;

use std::{error::Error, io::ErrorKind, pin::Pin, time::{SystemTime, UNIX_EPOCH}};

use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct CommanderServer {}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

#[tonic::async_trait]
impl pb::commander_server::Commander for CommanderServer {

    type ChannelStream = ResponseStream;

    async fn channel(
        &self,
        req: Request<Streaming<Message>>,
    ) -> EchoResult<Self::ChannelStream> {
        println!("EchoServer::bidirectional_streaming_echo");

        let mut in_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(128);

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be dropped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => tx
                        .send(Ok(Message { name: v.name, timestamp: 123, payload: Vec::new() }))
                        .await
                        .expect("working rx"),
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                eprintln!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was dropped
                        }
                    }
                }
            }
            println!("\tstream ended");
        });

        // echo just write the same data that was received
        let out_stream: ReceiverStream<Result<Message, Status>> = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::ChannelStream
        ))
    }
}

async fn send_welcome_message(tx: mpsc::Sender<Message>) {
    let _ = tx.send(Message {
        name: "handshake".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap(),
        payload: Vec::new(),
    }).await;
}

#[cfg(test)]
mod server_tests {
    use super::*;

    #[tokio::test]
    async fn test_welcome_message() {
        let (tx, mut rx) = mpsc::channel(1);
        send_welcome_message(tx).await;
        let actual = rx.recv().await;
        assert_eq!("handshake".to_string(), actual.unwrap().name);
    }
}
