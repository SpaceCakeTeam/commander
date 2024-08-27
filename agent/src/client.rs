use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::transport::Channel;

use messages::{
    build_message_or_print_error, 
    definitions::{Version, HANDSHAKE_COMMAND, VERSION_NAME_MESSAGE},
    pb::{commander_client::CommanderClient, Message},
    send2server,
    timenow,
};

const VERSION: &str = "1";

pub async fn agent_stream_manager(client: &mut CommanderClient<Channel>) {
    println!("|{}| agent started", timenow());

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
                println!("|{time}| received message {name}: {:#?}", std::str::from_utf8(&received.payload).ok().unwrap(), name=&received.name, time=&received.timestamp);

                let resp = get_response_message(received);
                match resp {
                    Some(message_to_send) => send2server(&mut tx, message_to_send).await,
                    _ => (),
                }

                println!("|{}| processed message", timenow());
            },
            None => {
                println!("|{}| Received None from stream :(", timenow());
                break;
            }
        }
    }

    println!("|{}| closing client!", timenow());
}

fn get_response_message(received_message: Message) -> Option<Message> {
    match received_message.name.as_str() {
        HANDSHAKE_COMMAND => Some(build_message_or_print_error(
            VERSION_NAME_MESSAGE, 
            &Version{ name: VERSION.to_string() },
        )),
        _ => None,
    }
}

#[cfg(test)]
mod client_tests {
    use messages::payload_serializer::{deserialize, serialize};

    use super::*;

    #[test]
    fn test_get_response_message() {
        let msg = Message{
            name: HANDSHAKE_COMMAND.to_string(),
            timestamp: timenow(),
            payload: serialize(&"ciao".to_string()).unwrap(),
        };

        let recv = get_response_message(msg);
        let message = recv.unwrap();
        assert_eq!(VERSION_NAME_MESSAGE, message.name);
        assert_eq!(VERSION, deserialize::<Version>(&message.payload).unwrap().name);
    }
}
