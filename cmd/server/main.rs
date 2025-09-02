pub mod server;

use crate::server::{ServerState, events_handler};
use metered_usage::infrastructure::clickhouse_client::ClickhouseClient;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let client = ClickhouseClient::new();

    let server_state = Arc::new(ServerState {
        clickhouse_client_creator: client,
    });

    let host = match env::var("HOST") {
        Ok(host) => host,
        Err(_) => "127.0.0.1".into(),
    };

    let api_url = format!("{host}:8000");

    let listener = tokio::net::TcpListener::bind(&api_url).await.unwrap();

    println!("Listening server in {}", api_url);

    axum::serve(listener, events_handler(server_state))
        .await
        .unwrap()
}
