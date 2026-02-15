// main.rs
//
// Same pattern as service1: connect, subscribe, reply.

use bytes::Bytes;
use common::{SUBJECT_TRANSFORM, TransformRequest, TransformResponse};
use tokio_stream::StreamExt;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\t[Service2] Transformation Service (NATS)");
    eprintln!("\t[Service2] Connecting to {NATS_URL}...");

    let client = async_nats::connect(NATS_URL).await?;
    eprintln!("\t[Service2] Connected. Subscribing to \"{SUBJECT_TRANSFORM}\"...");

    let mut subscription = client.subscribe(SUBJECT_TRANSFORM).await?;
    eprintln!("\t[Service2] Waiting for requests...");

    while let Some(message) = subscription.next().await {
        let request: TransformRequest = serde_json::from_slice(&message.payload)?;
        let response = transform(request);

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
