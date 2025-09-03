use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use metered_usage::services::metered_usage_service::MeteredUsageService;
use metered_usage::{
    infrastructure::clickhouse_client::ClickhouseClient, services::entities::MeteredUsageEvent,
};
use std::sync::Arc;

async fn root() -> &'static str {
    "Hello, Bacon!"
}

async fn save_event(State(api_state): State<Arc<ServerState>>) -> Result<Json<()>, ()> {
    println!("Creation service...");
    let service = MeteredUsageService::new(api_state.clickhouse_client_creator.clone());
    println!("Service created...");

    let operation_result = service
        .insert_metered_event(MeteredUsageEvent::random(), &service.db_client)
        .await;

    return match operation_result {
        Ok(_) => Ok(Json(())),
        Err(error) => {
            println!("error {}", error);
            return Err(());
        }
    };
}

pub struct ServerState {
    pub clickhouse_client_creator: ClickhouseClient,
}

pub fn events_handler(server_state: Arc<ServerState>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/", post(save_event))
        .with_state(server_state)
}
