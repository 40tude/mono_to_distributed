// Service1 — Processing service (subscribes to NATS subject "service.process")
//
// This service connects to the NATS broker, subscribes to a subject,
// and replies to each incoming request with a processed result.
// Compare with step06/service1 which used an HTTP server (Axum) instead.
//
// Rust guideline compliant 2025-05-14

use bytes::Bytes;
use common::{ProcessRequest, ProcessResponse, SUBJECT_PROCESS};
use tokio_stream::StreamExt;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

fn process(request: ProcessRequest) -> ProcessResponse {
    eprintln!("\t[Service1] Processing request: {}", request.request_id);
    eprintln!("\t[Service1] Input value: {}", request.value);

    // Business logic: multiply by 2 (same as every previous step)
    let result = request.value * 2;

    ProcessResponse {
        value: result,
        processed: true,
        request_id: request.request_id,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\t[Service1] Processing Service (NATS)");
    eprintln!("\t[Service1] Connecting to {NATS_URL}...");

    let client = async_nats::connect(NATS_URL).await?;
    eprintln!("\t[Service1] Connected. Subscribing to \"{SUBJECT_PROCESS}\"...");

    let mut subscription = client.subscribe(SUBJECT_PROCESS).await?;
    eprintln!("\t[Service1] Waiting for requests...");

    // Listen forever for incoming messages
    while let Some(message) = subscription.next().await {
        // Deserialize the JSON payload
        let request: ProcessRequest = serde_json::from_slice(&message.payload)?;

        // Process
        let response = process(request);

        // Reply — only if the message has a reply subject (request/reply pattern)
        if let Some(reply_subject) = message.reply {
            let response_bytes = Bytes::from(serde_json::to_vec(&response)?);
            client.publish(reply_subject, response_bytes).await?;
        }
    }

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
