use messages::{build_message_or_print_error, definitions::{ERROR_EVENT, K8S_GET_VERSION_EVENT}, pb::Message, timenow};

use kubernetes_client;

pub async fn retrieve_k8s_version_and_build_message() -> Message {
    println!("received k8s version request");

    let k8s_version_result = kubernetes_client::get_version().await;
    match k8s_version_result {
        Ok(v)=> {
            println!("|{}| Retrieved k8s version {:?}", timenow(), v);
            build_message_or_print_error(K8S_GET_VERSION_EVENT, &v)
        },
        Err(e)=>{
            println!("|{}| Something went wrong retrieving k8s version {:?}", timenow(), e);
            build_message_or_print_error(ERROR_EVENT, &e)
        }
    }
}