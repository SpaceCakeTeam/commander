use messages::{
    build_message_or_print_error,
    definitions::{HANDSHAKE_COMMAND, HEARTBEAT_EVENT, K8S_GET_VERSION_COMMAND, VERSION_NAME_MESSAGE},
    pb::{commander_server::Commander, Message},
    send2client,
    timenow
};

use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use std::{error::Error, io::ErrorKind, pin::Pin, sync::Arc, thread::sleep, time::Duration};

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
        println!("|{}| new client connected", timenow());

        let in_stream: Streaming<Message> = req.into_inner();
        let (mut tx, rx) = mpsc::channel(1);

        println!("|{}| sending welcome message", timenow());
        send2client(&mut tx, build_message_or_print_error(HANDSHAKE_COMMAND, b"")).await;
    
        let stream_reference_counter = Arc::new(tx);

        let server_event_manager_stream = Arc::clone(&stream_reference_counter);
        tokio::spawn(server_manager(in_stream, server_event_manager_stream));

        let heartbeat_manager_stream = Arc::clone(&stream_reference_counter);
        tokio::spawn(heartbeat_sender(heartbeat_manager_stream));

        let out_stream: ReceiverStream<Result<Message, Status>> = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as Self::ChannelStream))
    }
}

async fn heartbeat_sender(tx: Arc<Sender<Result<Message, Status>>>) {
    loop {
        sleep(Duration::from_secs(5));
        println!("|{}| sending heartbeat", timenow());
        let message = build_message_or_print_error(HEARTBEAT_EVENT, b"");
        // TODO: try to use send2client instead
        let _ = tx.send(Ok(message)).await;
    } 
}

async fn server_manager(mut in_stream: Streaming<Message>, tx: Arc<Sender<Result<Message, Status>>>) {
    while let Some(result) = in_stream.next().await {
        match result {
            Ok(v) => {
                println!("|{time}| received message {name}: {:#?}", std::str::from_utf8(&v.payload).ok().unwrap(), name=&v.name, time=&v.timestamp);
                process_message_and_response(v, tx.clone()).await;
            },
            Err(err) => {
                println!("|{time}| received error from client: {:#?}", err, time=timenow());
                if let Some(io_err) = match_for_io_error(&err) {
                    if io_err.kind() == ErrorKind::BrokenPipe {
                        eprintln!("\t|{time}| client disconnected: broken pipe", time=timenow());
                        break;
                    }
                }
            }
        }
    }
    println!("\t|{}| client stream ended", timenow());
}

async fn process_message_and_response(msg: Message, stream: Arc<Sender<Result<Message, Status>>>) {
    match msg.name.as_str() {
        VERSION_NAME_MESSAGE => version_name_message_responder(msg, stream).await,
        _ => {},
    }
}

async fn version_name_message_responder(received_message: Message, str: Arc<Sender<Result<Message, Status>>>) {
    println!("Processing {}", received_message.name);
    let message: Message = Message{name: K8S_GET_VERSION_COMMAND.to_string(),timestamp: timenow(), payload: Vec::new()};

    // TODO: try to use send2client instead
    let _ = str.send(Ok(message)).await;
}