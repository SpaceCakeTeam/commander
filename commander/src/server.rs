pub mod pb {
    tonic::include_proto!("messages");
}

use pb::Message;
use tokio::sync::mpsc::{self, Sender};

use std::{error::Error, io::ErrorKind, pin::Pin, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

type ChannelResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct CommanderServer {
    // tx: Sender<Result<Message, Status>>,
}

// trait Manager {
//     async fn manager(&mut self);
// }


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

fn timenow() -> u128 {
    return SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .try_into()
    .unwrap()
}

#[tonic::async_trait]
impl pb::commander_server::Commander for CommanderServer {

    type ChannelStream = ResponseStream;

    async fn channel(
        &self,
        req: Request<Streaming<Message>>,
    ) -> ChannelResult<Self::ChannelStream> {
        println!("new client connected");

        let mut in_stream: Streaming<Message> = req.into_inner();
        let (mut tx, rx) = mpsc::channel(1);

        // let mut tx1 = tx.clone();
        let out_stream: ReceiverStream<Result<Message, Status>> = ReceiverStream::new(rx);

        println!("sending welcome message");
        send_welcome_message(&mut tx).await;

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        println!("received message from client {:#?}: {:#?}", timenow(), v);
                        // sleep(Duration::from_secs(10));
                        // send_welcome_message(&mut tx).await;
                        // tx
                        //     .send(Ok(Message { name: v.name, timestamp: 123, payload: Vec::new() }))
                        //     .await
                        //     .expect("working rx")
                    },
                    Err(err) => {
                        println!("received error from client {:#?}: {:#?}", timenow(), err);
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                eprintln!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }
                    }
                }
            }
            println!("\tclient stream ended {:#?}", timenow());
        });

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(5));
                println!("sending heartbeat");
                send_heartbeat(&mut tx).await;
            }
        });

        Ok(Response::new(Box::pin(out_stream) as Self::ChannelStream))
    }
}

async fn send_welcome_message(tx: &mut Sender<Result<Message, Status>>) {
    let _ = tx.send(Ok(Message {
        name: "handshake".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap(),
        payload: Vec::new(),
    })).await;
}

async fn send_heartbeat(tx: &mut Sender<Result<Message, Status>>) {
    let _ = tx.send(Ok(Message {
        name: "heartbeat".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap(),
        payload: Vec::new(),
    })).await;
}

#[cfg(test)]
mod server_tests {
    use super::*;

    #[tokio::test]
    async fn test_welcome_message() {
        let (mut tx, mut rx) = mpsc::channel(1);
        send_welcome_message(&mut tx).await;
        let actual = rx.recv().await;
        assert_eq!("handshake".to_string(), actual.unwrap().unwrap().name);
    }
}
