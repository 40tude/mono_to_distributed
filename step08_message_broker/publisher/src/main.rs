// Publisher — Orchestrates the pipeline via NATS request/reply
//
// Replaces the HTTP-based orchestrator from step06.
// Instead of sending HTTP POST requests to known URLs, the publisher sends
// messages to NATS *subjects*. It does not know (or care) which process
// handles them — the broker routes the messages.
//
// Rust guideline compliant 2025-05-14

use bytes::Bytes;
use common::{
    ProcessRequest, ProcessResponse, SUBJECT_PROCESS, SUBJECT_TRANSFORM, TransformRequest,
    TransformResponse,
};
use std::time::Duration;
use uuid::Uuid;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

/// How long to wait for a service reply before giving up.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPhase 07: Message Broker Pipeline (NATS)\n");

    println!("[Publisher] Connecting to {NATS_URL}...");
    let client = async_nats::connect(NATS_URL).await?;
    println!("[Publisher] Connected to NATS broker");

    println!("\n--- Processing Distributed Pipeline ---");

    let request_id = Uuid::new_v4().to_string();
    let input_value = 42;

    println!("[Publisher] Starting request: {request_id}");
    println!("[Publisher] Input value: {input_value}");

    // Step 1: Send to Service1 via NATS subject "service.process"
    println!("\n[Publisher] Sending to \"{SUBJECT_PROCESS}\"...");
    let process_request = ProcessRequest {
        value: input_value,
        request_id: request_id.clone(),
    };
    let payload = Bytes::from(serde_json::to_vec(&process_request)?);

    // client.request() sends the message and waits for a reply
    // Internally, NATS creates a temporary "inbox" subject for the response
    let reply = tokio::time::timeout(REQUEST_TIMEOUT, client.request(SUBJECT_PROCESS, payload))
        .await??;

    let result1: ProcessResponse = serde_json::from_slice(&reply.payload)?;
    println!(
        "[Publisher] Service1 result: value={}, processed={}",
        result1.value, result1.processed
    );

    // Step 2: Send to Service2 via NATS subject "service.transform"
    println!("\n[Publisher] Sending to \"{SUBJECT_TRANSFORM}\"...");
    let transform_request = TransformRequest {
        value: result1.value,
        request_id: request_id.clone(),
    };
    let payload = Bytes::from(serde_json::to_vec(&transform_request)?);

    let reply = tokio::time::timeout(
        REQUEST_TIMEOUT,
        client.request(SUBJECT_TRANSFORM, payload),
    )
    .await??;

    let result2: TransformResponse = serde_json::from_slice(&reply.payload)?;
    println!(
        "[Publisher] Service2 result: original={}, transformed={}",
        result2.original, result2.transformed
    );

    println!("\n[Publisher] Pipeline completed successfully!");
    println!("[Publisher] Final result: {}", result2.transformed);

    println!("\nExecution complete");

    Ok(())
}
