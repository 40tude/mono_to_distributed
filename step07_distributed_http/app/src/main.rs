// main.rs

use common::{ProcessRequest, ProcessResponse, TransformRequest, TransformResponse};
use uuid::Uuid;

/// Base URL for service1 (processing).
const SERVICE1_URL: &str = "http://127.0.0.1:3001";

/// Base URL for service2 (transformation).
const SERVICE2_URL: &str = "http://127.0.0.1:3002";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPhase 07: Distributed system (HTTP)\n");

    let client = reqwest::Client::new();

    println!("\n--- Processing Distributed Pipeline ---");

    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();
    let input_value = 42;

    println!("[App] Starting request: {request_id}");
    println!("[App] Input value: {input_value}");

    // Step 1: Send to Service1 (processing)
    println!("\n[App] Sending to Service1...");
    let process_request = ProcessRequest {
        value: input_value,
        request_id: request_id.clone(),
    };

    let result1: ProcessResponse = client
        .post(format!("{SERVICE1_URL}/process"))
        .json(&process_request)
        .send()
        .await?
        .json()
        .await?;

    println!(
        "[App] Service1 result: value={}, processed={}",
        result1.value, result1.processed
    );

    // Step 2: Send to Service2 (transformation)
    println!("\n[App] Sending to Service2...");
    let transform_request = TransformRequest {
        value: result1.value,
        request_id: request_id.clone(),
    };

    let result2: TransformResponse = client
        .post(format!("{SERVICE2_URL}/transform"))
        .json(&transform_request)
        .send()
        .await?
        .json()
        .await?;

    println!(
        "[App] Service2 result: original={}, transformed={}",
        result2.original, result2.transformed
    );

    println!("\n[App] Pipeline completed successfully!");
    println!("[App] Final result: {}", result2.transformed);

    println!("\nExecution complete");

    Ok(())
}
