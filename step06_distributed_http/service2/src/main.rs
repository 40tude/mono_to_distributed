// main.rs

use axum::routing::{get, post};
use axum::{Json, Router};
use common::{TransformRequest, TransformResponse};
use tokio::net::TcpListener;

/// Port on which this transformation service listens.
const PORT: u16 = 3002;

fn transform(request: TransformRequest) -> TransformResponse {
    eprintln!("[Service2] Transforming request: {}", request.request_id);
    eprintln!("[Service2] Input value: {}", request.value);

    let transformed = format!("Value-{:04}", request.value);

    TransformResponse {
        original: request.value,
        transformed,
        request_id: request.request_id,
    }
}

async fn handle_transform(Json(request): Json<TransformRequest>) -> Json<TransformResponse> {
    Json(transform(request))
}

async fn handle_health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    eprintln!("Service 2: Transformation Service");

    let app = Router::new()
        .route("/transform", post(handle_transform))
        .route("/health", get(handle_health));

    let addr = format!("0.0.0.0:{PORT}");
    eprintln!("[Service2] Listening on http://{addr}");

    let listener = TcpListener::bind(&addr).await.expect("failed to bind port");

    axum::serve(listener, app).await.expect("server error");
}

