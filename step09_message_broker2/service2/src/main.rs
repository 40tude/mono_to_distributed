// main.rs
//
// Same pattern as service1: connect, subscribe, reply.

use bytes::Bytes;
use common::{QUEUE_TRANSFORM, SUBJECT_TRANSFORM, TransformRequest, TransformResponse};
use tokio_stream::StreamExt;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

/// Maximum absolute value that fits in the 4-digit format `Value-XXXX`.
const MAX_TRANSFORM_VALUE: i32 = 9999;

fn transform(request: TransformRequest) -> TransformResponse {
    eprintln!("\t[Service2] Transforming request: {}", request.request_id);
    eprintln!("\t[Service2] Input value: {}", request.value);

    if request.value.abs() > MAX_TRANSFORM_VALUE {
        let msg = format!(
            "value {} exceeds 4-digit format range [-{MAX_TRANSFORM_VALUE}..{MAX_TRANSFORM_VALUE}]",
            request.value
        );
        eprintln!("\t[Service2] {msg}");
        return TransformResponse {
            original: request.value,
            transformed: String::new(),
            request_id: request.request_id,
            error: Some(msg),
        };
    }

    let transformed = format!("Value-{:04}", request.value);

    TransformResponse {
        original: request.value,
        transformed,
        request_id: request.request_id,
        error: None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\t[Service2] Transformation Service (NATS)");
    eprintln!("\t[Service2] Connecting to {NATS_URL}...");

    let client = async_nats::connect(NATS_URL).await?;
    eprintln!("\t[Service2] Connected. Subscribing to \"{SUBJECT_TRANSFORM}\"...");

    let mut subscription = client
        .queue_subscribe(SUBJECT_TRANSFORM, QUEUE_TRANSFORM.into())
        .await?;
    eprintln!("\t[Service2] Waiting for requests...");

    // Race between incoming messages and CTRL+C for graceful shutdown
    let shutdown = tokio::signal::ctrl_c();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            msg = subscription.next() => {
                match msg {
                    Some(message) => {
                        let request: TransformRequest = serde_json::from_slice(&message.payload)?;
                        let response = transform(request);
                        if let Some(reply_subject) = message.reply {
                            let response_bytes = Bytes::from(serde_json::to_vec(&response)?);
                            client.publish(reply_subject, response_bytes).await?;
                        }
                    }
                    None => break,
                }
            }
            _ = &mut shutdown => {
                eprintln!("\t[Service2] Initiating Shutdown...");
                break;
            }
        }
    }

    eprintln!("\t[Service2] Server Exiting.");
    Ok(())
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
        assert!(response.error.is_none());
    }

    #[test]
    fn transform_pads_small_value() {
        let request = TransformRequest {
            value: 7,
            request_id: "test-2".to_string(),
        };
        let response = transform(request);
        assert_eq!(response.transformed, "Value-0007");
        assert!(response.error.is_none());
    }

    #[test]
    fn value_exceeding_4_digits_returns_error() {
        let request = TransformRequest {
            value: 10_000,
            request_id: "test-overflow".to_string(),
        };
        let response = transform(request);
        assert!(response.error.is_some());
        assert!(response.transformed.is_empty());
    }

    #[test]
    fn negative_value_exceeding_4_digits_returns_error() {
        let request = TransformRequest {
            value: -10_000,
            request_id: "test-neg-overflow".to_string(),
        };
        let response = transform(request);
        assert!(response.error.is_some());
    }
}
