// main.rs

use axum::routing::{get, post};
use axum::{Json, Router};
use common::{ProcessRequest, ProcessResponse};
use tokio::net::TcpListener;

/// Port on which this processing service listens.
const PORT: u16 = 3001;

fn process(request: ProcessRequest) -> ProcessResponse {
    eprintln!("\t[Service1] Processing request: {}", request.request_id);
    eprintln!("\t[Service1] Input value: {}", request.value);

    // Business logic: multiply by 2
    let result = request.value * 2;

    ProcessResponse {
        value: result,
        processed: true,
        request_id: request.request_id,
    }
}

async fn handle_process(Json(request): Json<ProcessRequest>) -> Json<ProcessResponse> {
    Json(process(request))
}

async fn handle_health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    eprintln!("\t[Service1] Processing  Service");

    let app = Router::new()
        .route("/process", post(handle_process))
        .route("/health", get(handle_health));

    let addr = format!("0.0.0.0:{PORT}");
    eprintln!("\t[Service1] Listening on http://{addr}");

    let listener = TcpListener::bind(&addr).await.expect("failed to bind port");

    axum::serve(listener, app).await.expect("server error");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn negative_value_stays_negative() {
        let request = ProcessRequest {
            value: -18,
            request_id: "test-1".to_string(),
        };
        let response = process(request);
        assert!(response.processed);
        assert_eq!(response.value, -36);
    }

    #[test]
    fn process_doubles_value() {
        let request = ProcessRequest {
            value: 21,
            request_id: "test-2".to_string(),
        };
        let response = process(request);
        assert_eq!(response.value, 42);
        assert!(response.processed);
    }
}
