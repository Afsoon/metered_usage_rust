use axum::{
    Router,
    extract::{Json, MatchedPath, State, rejection::JsonRejection},
    http::Request,
    routing::{get, post},
};
use metered_usage::services::metered_usage_service::MeteredUsageService;
use metered_usage::{
    infrastructure::clickhouse_client::ClickhouseClient, services::entities::MeteredUsageEvent,
};
use std::sync::Arc;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, debug, error, info, info_span, instrument};
use uuid::Uuid;

use crate::entities::MeteredUsagePayload;

async fn root() -> &'static str {
    return "Hello, Bacon!";
}

#[instrument(skip_all)]
async fn save_event(
    State(api_state): State<Arc<ServerState>>,
    payload: Result<Json<MeteredUsagePayload>, JsonRejection>,
) -> Result<Json<()>, ()> {
    debug!("Creation service...");
    let service = MeteredUsageService::new(api_state.clickhouse_client_creator.clone());
    debug!("Service created...");

    info!("is empty {:?}", payload.is_err());
    let event_to_create: MeteredUsageEvent = match payload {
        Ok(Json(body)) => body.into(),
        Err(err) => {
            error!("Unable to process body, {:?}", err);
            MeteredUsageEvent::random()
        }
    };

    let operation_result = service
        .insert_metered_event(event_to_create, &service.db_client)
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    let span_id = Uuid::now_v7();

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        id = span_id.hyphenated().to_string(),
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
}
