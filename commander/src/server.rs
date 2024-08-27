use messages::{build_message_or_print_error, messages::{HANDSHAKE_COMMAND, HEARTBEAT_EVENT}, pb::{commander_server::Commander, Message}, send2client, timenow};
use tokio::sync::mpsc;

use std::{error::Error, io::ErrorKind, pin::Pin, thread::sleep, time::Duration};

use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

type ChannelResult<T> = Result<Response<T>, Status>;

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
impl Commander for CommanderServer {

    type ChannelStream = ResponseStream;

    async fn channel(
        &self,
        req: Request<Streaming<Message>>,
    ) -> ChannelResult<Self::ChannelStream> {
        println!("new client connected");

        let mut in_stream: Streaming<Message> = req.into_inner();
        let (mut tx, rx) = mpsc::channel(1);

        let out_stream: ReceiverStream<Result<Message, Status>> = ReceiverStream::new(rx);

        println!("sending welcome message");
        send2client(&mut tx, build_message_or_print_error(HANDSHAKE_COMMAND, b"")).await;

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        println!("received message from client {:#?}: {:#?}", timenow(), v);
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
                send2client(&mut tx, build_message_or_print_error(HEARTBEAT_EVENT, b"")).await
            }
        });

        Ok(Response::new(Box::pin(out_stream) as Self::ChannelStream))
    }
}
