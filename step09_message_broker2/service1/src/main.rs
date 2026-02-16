// main.rs
//
// This service connects to the NATS broker, subscribes to a subject,
// and replies to each incoming request with a processed result.

use bytes::Bytes;
use common::{
    ProcessRequest, ProcessResponse, QUEUE_PROCESS, SUBJECT_PROCESS, SUBJECT_VERSION_SERVICE1,
    VersionRequest, VersionResponse,
};
use tokio_stream::StreamExt;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

fn process(request: ProcessRequest) -> ProcessResponse {
    eprintln!("\t[Service1] Processing request: {}", request.request_id);
    eprintln!("\t[Service1] Input value: {}", request.value);

    // Business logic: multiply by 2 (same as every previous step)
    match request.value.checked_mul(2) {
        Some(result) => ProcessResponse {
            value: result,
            processed: true,
            request_id: request.request_id,
            error: None,
        },
        None => {
            let msg = format!("overflow: {} * 2 exceeds i32 range", request.value);
            eprintln!("\t[Service1] {msg}");
            ProcessResponse {
                value: request.value,
                processed: false,
                request_id: request.request_id,
                error: Some(msg),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\t[Service1] Processing Service (NATS)");
    eprintln!("\t[Service1] Connecting to {NATS_URL}...");

    let client = async_nats::connect(NATS_URL).await?;
    eprintln!("\t[Service1] Connected. Subscribing to \"{SUBJECT_PROCESS}\"...");

    let mut subscription = client
        .queue_subscribe(SUBJECT_PROCESS, QUEUE_PROCESS.into())
        .await?;

    let mut version_sub = client.subscribe(SUBJECT_VERSION_SERVICE1).await?;
    eprintln!("\t[Service1] Waiting for requests...");

    // Race between incoming messages and CTRL+C for graceful shutdown
    let shutdown = tokio::signal::ctrl_c();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            msg = subscription.next() => {
                match msg {
                    Some(message) => {
                        let request: ProcessRequest = serde_json::from_slice(&message.payload)?;
                        let response = process(request);
                        if let Some(reply_subject) = message.reply {
                            let response_bytes = Bytes::from(serde_json::to_vec(&response)?);
                            client.publish(reply_subject, response_bytes).await?;
                        }
                    }
                    None => break,
                }
            }
            msg = version_sub.next() => {
                if let Some(message) = msg {
                    let req: VersionRequest = serde_json::from_slice(&message.payload)?;
                    let resp = VersionResponse {
                        service_name: "Service1".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string(),
                        request_id: req.request_id,
                    };
                    if let Some(reply_subject) = message.reply {
                        let bytes = Bytes::from(serde_json::to_vec(&resp)?);
                        client.publish(reply_subject, bytes).await?;
                    }
                }
            }
            _ = &mut shutdown => {
                eprintln!("\t[Service1] Initiating Shutdown...");
                break;
            }
        }
    }

    eprintln!("\t[Service1] Server Exiting.");
    Ok(())
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
        assert!(response.error.is_none());
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
        assert!(response.error.is_none());
    }

    #[test]
    fn overflow_returns_error() {
        let request = ProcessRequest {
            value: i32::MAX / 2 + 1,
            request_id: "test-overflow".to_string(),
        };
        let response = process(request);
        assert!(!response.processed);
        assert_eq!(response.value, i32::MAX / 2 + 1);
        assert!(response.error.is_some());
    }

    #[test]
    fn negative_overflow_returns_error() {
        let request = ProcessRequest {
            value: i32::MIN / 2 - 1,
            request_id: "test-neg-overflow".to_string(),
        };
        let response = process(request);
        assert!(!response.processed);
        assert!(response.error.is_some());
    }
}
