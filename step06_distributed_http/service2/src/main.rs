// main.rs

use axum::routing::{get, post};
use axum::{Json, Router};
use common::{TransformRequest, TransformResponse};
use tokio::net::TcpListener;

/// Port on which this transformation service listens.
const PORT: u16 = 3002;

fn transform(request: TransformRequest) -> TransformResponse {
    eprintln!("\t[Service2] Transforming request: {}", request.request_id);
    eprintln!("\t[Service2] Input value: {}", request.value);

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
    eprintln!("\t[Service2] Transformation Service");

    let app = Router::new()
        .route("/transform", post(handle_transform))
        .route("/health", get(handle_health));

    let addr = format!("0.0.0.0:{PORT}");
    eprintln!("\t[Service2] Listening on http://{addr}");

    let listener = TcpListener::bind(&addr).await.expect("failed to bind port");

    axum::serve(listener, app).await.expect("server error");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transform_formats_value() {
        let request = TransformRequest {
            value: 42,
            request_id: "test-1".to_string(),
        };
        let response = transform(request);
        assert_eq!(response.original, 42);
        assert_eq!(response.transformed, "Value-0042");
    }

    #[test]
    fn transform_pads_small_value() {
        let request = TransformRequest {
            value: 7,
            request_id: "test-2".to_string(),
        };
        let response = transform(request);
        assert_eq!(response.transformed, "Value-0007");
    }
}
