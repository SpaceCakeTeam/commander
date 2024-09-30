use axum::{routing::get, Router};

pub struct CommanderAPI {
    bind_address: String,
}

impl CommanderAPI {
    pub fn new(
        http_port: i64
    ) -> Self {
        Self{
            bind_address: format!("0.0.0.0:{}", http_port)
        }
    }

    pub async fn start(&self) {
        let app = Router::new()
            // FIXME: user must provide an id for the desired connection for which the version shall be required!
            .route("/version", get(version_handler));

        let listener = tokio::net::TcpListener::bind(self.bind_address.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
        // TODO: handle server close errors etc...
    }
}

// TODO: move somewhere else this handler, possibly in a Controller struct?
pub async fn version_handler() -> String {
    "Hello, World!".to_string()

    // let response = commander_server.send(Version_Command).await;
}