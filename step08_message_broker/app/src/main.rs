// main.rs
//
// Instead of sending HTTP POST requests to known URLs, the publisher sends
// messages to NATS *subjects*. It does not know (or care) which process handles
// them — the broker routes the messages.
//
// The pipeline function is generic over `Messaging` (DIP): it never touches
// NATS types directly. Only `NatsMessaging` knows about the concrete broker.

use bytes::Bytes;
use common::{
    Messaging, ProcessRequest, ProcessResponse, SUBJECT_PROCESS, SUBJECT_TRANSFORM,
    TransformRequest, TransformResponse,
};
use std::time::Duration;
use uuid::Uuid;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

/// How long to wait for a service reply before giving up.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n\nPhase 08: Message Broker Pipeline (NATS)\n");

    println!("[App] Connecting to {NATS_URL}...");
    let broker = NatsMessaging::connect(NATS_URL).await?;
    println!("[App] Connected to NATS broker");

    println!("\n--- Processing Distributed Pipeline ---");

    run_pipeline(&broker, 42).await?;

    println!("\nExecution complete");

    Ok(())
}

// Concrete NATS adapter (the only place that knows about async_nats)
struct NatsMessaging {
    client: async_nats::Client,
}

impl NatsMessaging {
    async fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = async_nats::connect(url).await?;
        Ok(Self { client })
    }
}

impl Messaging for NatsMessaging {
    async fn request(
        &self,
        subject: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let subject = subject.to_owned();
        let reply = tokio::time::timeout(
            REQUEST_TIMEOUT,
            self.client.request(subject, Bytes::from(payload)),
        )
        .await??;
        Ok(reply.payload.to_vec())
    }
}

// Broker-agnostic pipeline (depends on `Messaging`, not on NATS)
async fn run_pipeline(
    broker: &impl Messaging,
    input_value: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request_id = Uuid::new_v4().to_string();

    println!("[App] Starting request: {request_id}");
    println!("[App] Input value: {input_value}");

    // Step 1: Send to Service1 via subject "service.process"
    println!("\n[App] Sending to \"{SUBJECT_PROCESS}\"...");
    let process_request = ProcessRequest {
        value: input_value,
        request_id: request_id.clone(),
    };
    let payload = serde_json::to_vec(&process_request)?;

    let reply = broker.request(SUBJECT_PROCESS, payload).await?;
    let result1: ProcessResponse = serde_json::from_slice(&reply)?;
    println!(
        "[App] Service1 result: value={}, processed={}",
        result1.value, result1.processed
    );

    // Step 2: Send to Service2 via subject "service.transform"
    println!("\n[App] Sending to \"{SUBJECT_TRANSFORM}\"...");
    let transform_request = TransformRequest {
        value: result1.value,
        request_id: request_id.clone(),
    };
    let payload = serde_json::to_vec(&transform_request)?;

    let reply = broker.request(SUBJECT_TRANSFORM, payload).await?;
    let result2: TransformResponse = serde_json::from_slice(&reply)?;
    println!(
        "[App] Service2 result: original={}, transformed={}",
        result2.original, result2.transformed
    );

    println!("\n[App] Pipeline completed successfully!");
    println!("[App] Final result: {}", result2.transformed);

    Ok(())
}
