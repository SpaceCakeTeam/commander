use messages::{
    build_message_or_print_error,
    definitions::{HANDSHAKE_COMMAND, HEARTBEAT_EVENT, K8S_GET_VERSION_COMMAND, VERSION_NAME_MESSAGE},
    pb::{commander_server::Commander, Message},
    send2client,
    timenow,
    uuidv4
};

use tokio::{sync::mpsc, task::JoinHandle};
use tokio::sync::mpsc::Sender;

use std::{pin::Pin, sync::Arc, time::Duration};

use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send>>;

type ChannelResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct CommanderServer {}

#[tonic::async_trait]
impl Commander for CommanderServer {
    type ChannelStream = ResponseStream;

    async fn channel(
        &self,
        req: Request<Streaming<Message>>,
    ) -> ChannelResult<Self::ChannelStream> {
        let ch_id = uuidv4();
        println!("|{}| new client connected {}", timenow(), ch_id);

        let in_stream: Streaming<Message> = req.into_inner();
        let (mut tx, rx) = mpsc::channel(1);

        println!("|{}| sending welcome message {}", timenow(), ch_id);
        send2client(&mut tx, build_message_or_print_error(HANDSHAKE_COMMAND, b"")).await;

        let stream_reference_counter = Arc::new(tx.clone());

        let heartbeat_manager_stream = Arc::clone(&stream_reference_counter);
        let heartbeat_handle = tokio::spawn(heartbeat_sender(heartbeat_manager_stream, ch_id));

        let server_event_manager_stream = Arc::clone(&stream_reference_counter);
        tokio::spawn(server_manager(in_stream, server_event_manager_stream, heartbeat_handle));

        let out_stream: ReceiverStream<Result<Message, Status>> = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as Self::ChannelStream))
    }
}

async fn heartbeat_sender(tx: Arc<Sender<Result<Message, Status>>>, ch_id: String) {
    println!("|{}| setup heartbeat {}", timenow(), ch_id);
    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                println!("|{}| sending heartbeat {}", timenow(), ch_id);
                let message = build_message_or_print_error(HEARTBEAT_EVENT, b"");
                // TODO: try to use send2client instead
                let sent = tx.send(Ok(message)).await;
                if sent.is_err() {
                    println!("|{}| failed to send heartbeat {} err {:#?}", timenow(), ch_id, sent.unwrap_err());
                }
            },
            else => {
                println!("|{}| stopping heartbeat {}", timenow(), ch_id);
                break;
            }
        }
    }
}

async fn server_manager(mut in_stream: Streaming<Message>, tx: Arc<Sender<Result<Message, Status>>>, join_handle: JoinHandle<()>) {
    while let Some(result) = in_stream.next().await {
        match result {
            Ok(v) => {
                println!("|{time}| received message {name}: {:#?}", std::str::from_utf8(&v.payload).ok().unwrap(), name=&v.name, time=&v.timestamp);
                process_message_and_response(v, tx.clone()).await;
            },
            Err(err) => {
                println!("|{time}| received error from client: {:#?}", err, time=timenow());
                join_handle.abort();
                println!("|{time}| handle aborted", time=timenow());
                break;
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
