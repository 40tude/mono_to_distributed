// main.rs
//
// Instead of sending HTTP POST requests to known URLs, the publisher sends
// messages to NATS *subjects*. It does not know (or care) which process handles
// them — the broker routes the messages.
//
// The pipeline function is generic over `Messaging` (DIP): it never touches
// NATS types directly. Only `NatsMessaging` knows about the concrete broker.
//
// Multiple input values are processed concurrently via a JoinSet, mirroring
// the approach used in step07's multi-value variant.

use bytes::Bytes;
use common::{
    Messaging, ProcessRequest, ProcessResponse, SUBJECT_PROCESS, SUBJECT_TRANSFORM,
    SUBJECT_VERSION_SERVICE1, SUBJECT_VERSION_SERVICE2, TransformRequest, TransformResponse,
    VersionRequest, VersionResponse,
};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// NATS server URL (default local port).
const NATS_URL: &str = "nats://127.0.0.1:4222";

/// How long to wait for a service reply before giving up.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Input values fed concurrently into the pipeline.
/// Includes edge cases: 5_000 will overflow the 4-digit format after doubling,
/// and 1_073_741_824 (i32::MAX/2 + 1) will overflow on multiply.
const INPUT_VALUES: [i32; 5] = [10, 42, 5_000, 50, 1_073_741_824];

// ---------------------------------------------------------------------------
// Concrete NATS adapter (the only place that knows about async_nats)
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Broker-agnostic pipeline (depends on `Messaging`, not on NATS)
// ---------------------------------------------------------------------------

/// Runs the full pipeline for a single value: service1 (process) then service2 (transform).
/// Returns `(original_value, transformed_string, request_id)`.
async fn run_pipeline(
    broker: &(impl Messaging + Sync),
    input_value: i32,
    request_id: String,
) -> Result<(i32, String, String), Box<dyn std::error::Error + Send + Sync>> {
    let short_id = &request_id[..8];

    // Step 1: Send to Service1 via subject "service.process"
    let process_request = ProcessRequest {
        value: input_value,
        request_id: request_id.clone(),
    };
    let payload = serde_json::to_vec(&process_request)?;

    let reply = broker.request(SUBJECT_PROCESS, payload).await?;
    let result1: ProcessResponse = serde_json::from_slice(&reply)?;

    if let Some(err) = &result1.error {
        println!("[{short_id}] Service1 ERROR: {err}");
        return Ok((input_value, format!("ERROR(service1): {err}"), request_id));
    }
    println!(
        "[{short_id}] Service1 done: value={input_value} -> processed={}",
        result1.value
    );

    // Step 2: Send to Service2 via subject "service.transform"
    let transform_request = TransformRequest {
        value: result1.value,
        request_id: request_id.clone(),
    };
    let payload = serde_json::to_vec(&transform_request)?;

    let reply = broker.request(SUBJECT_TRANSFORM, payload).await?;
    let result2: TransformResponse = serde_json::from_slice(&reply)?;

    if let Some(err) = &result2.error {
        println!("[{short_id}] Service2 ERROR: {err}");
        return Ok((input_value, format!("ERROR(service2): {err}"), request_id));
    }
    println!(
        "[{short_id}] Service2 done: value={} -> transformed={}",
        result2.original, result2.transformed
    );

    Ok((input_value, result2.transformed, request_id))
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n\nPhase 09: Message Broker Pipeline II\n");

    println!("[App] Connecting to {NATS_URL}...");
    let broker = Arc::new(NatsMessaging::connect(NATS_URL).await?);
    println!("[App] Connected to NATS broker");

    // Query service versions
    let ver_id = Uuid::new_v4().to_string();
    let ver_req = serde_json::to_vec(&VersionRequest {
        request_id: ver_id,
    })?;

    let reply1 = broker.request(SUBJECT_VERSION_SERVICE1, ver_req.clone()).await?;
    let v1: VersionResponse = serde_json::from_slice(&reply1)?;
    println!("[App] {} version: {}", v1.service_name, v1.version);

    let reply2 = broker.request(SUBJECT_VERSION_SERVICE2, ver_req).await?;
    let v2: VersionResponse = serde_json::from_slice(&reply2)?;
    println!("[App] {} version: {}", v2.service_name, v2.version);

    println!(
        "\n--- Processing Distributed Pipeline ({} values concurrently) ---\n",
        INPUT_VALUES.len()
    );

    // Spawn one task per input value
    let mut join_set = tokio::task::JoinSet::new();

    for &value in &INPUT_VALUES {
        let request_id = Uuid::new_v4().to_string();
        let short_id = &request_id[..8];
        println!("[App] Spawning pipeline for value={value}  request_id={short_id}");

        let broker = Arc::clone(&broker);
        join_set.spawn(async move { run_pipeline(&*broker, value, request_id).await });
    }

    println!();

    // Collect results as they complete (arbitrary order)
    let mut results: Vec<(i32, String, String)> = Vec::with_capacity(INPUT_VALUES.len());

    while let Some(outcome) = join_set.join_next().await {
        let (value, transformed, request_id) = outcome??;
        let short_id = &request_id[..8];
        println!("[App] Completed: value={value} -> {transformed}  request_id={short_id}");
        results.push((value, transformed, request_id));
    }

    // Sort by input value for clean display
    results.sort_by_key(|(value, _, _)| *value);

    println!("\n--- Final Results (sorted by input value) ---");
    for (value, transformed, request_id) in &results {
        let short_id = &request_id[..8];
        println!("  {value} -> {transformed}  [{short_id}]");
    }

    println!("\nExecution complete");

    Ok(())
}
