
use messages::{build_message_or_print_error, definitions::Version, pb::Message};

use kubernetes_client;

pub async fn retrieve_k8s_version_and_build_message() -> Option<Message> {
    println!("received k8s version request");

    let k8s_version_result = kubernetes_client::get_version().await;
    match k8s_version_result {
        Ok(v)=>println!("SOme {:?}",v),
        Err(e)=>println!("NONE {:?}",e)
    };
    Some(build_message_or_print_error("MY NAME FAKE FOR TESTING", &Version{name:"fake".to_string()}))
}