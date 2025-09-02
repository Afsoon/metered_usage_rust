pub mod server;

use crate::server::{ServerState, events_handler};
use metered_usage::infrastructure::clickhouse_client::ClickhouseClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let client = ClickhouseClient::new();

    let server_state = Arc::new(ServerState {
        clickhouse_client_creator: client,
    });

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Listening server in 127.0.0.1:8000");

    axum::serve(listner, events_handler(server_state))
        .await
        .unwrap()
}
