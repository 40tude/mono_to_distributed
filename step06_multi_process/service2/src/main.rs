// main.rs

use common::{Message, TransformRequest, TransformResponse};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct TransformationService;

impl TransformationService {
    fn new() -> Self {
        eprintln!("\t[Service2] Initialized - Ready to transform requests");
        TransformationService
    }

    fn transform(&self, request: TransformRequest) -> TransformResponse {
        eprintln!("\t[Service2] Transforming request: {}", request.request_id);
        eprintln!("\t[Service2] Input value: {}", request.value);

        // Transform the value to a formatted string
        let transformed = format!("Value-{:04}", request.value);

        TransformResponse {
            original: request.value,
            transformed,
            request_id: request.request_id,
        }
    }
}

fn main() {
    eprintln!("\t[Service2] Transformation Service");
    eprintln!("\t[Service2] Listening on STDIN for JSON messages...");

    let service = TransformationService::new();
    let running = Arc::new(AtomicBool::new(true));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        if !running.load(Ordering::Relaxed) {
            break;
        }

        match line {
            Ok(json_str) => {
                if json_str.trim().is_empty() {
                    continue;
                }

                match Message::from_json(&json_str) {
                    Ok(Message::Transform(request)) => {
                        let response = service.transform(request);
                        let message = Message::TransformResult(response);
                        let json = message.to_json();

                        writeln!(stdout, "{}", json).unwrap();
                        stdout.flush().unwrap();
                    }
                    Ok(Message::Shutdown) => {
                        eprintln!("\t[Service2] Shutdown signal received");
                        running.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {
                        eprintln!("\t[Service2] Unexpected message type");
                    }
                    Err(e) => {
                        eprintln!("\t[Service2] Failed to parse message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("\t[Service2] Error reading line: {}", e);
                break;
            }
        }
    }

    eprintln!("\t[Service2] Shutting down");
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TransformRequest;

    #[test]
    fn transform_formats_value() {
        let service = TransformationService::new();
        let request = TransformRequest {
            value: 42,
            request_id: "test-1".to_string(),
        };
        let response = service.transform(request);
        assert_eq!(response.original, 42);
        assert_eq!(response.transformed, "Value-0042");
    }

    #[test]
    fn transform_pads_small_value() {
        let service = TransformationService::new();
        let request = TransformRequest {
            value: 7,
            request_id: "test-2".to_string(),
        };
        let response = service.transform(request);
        assert_eq!(response.transformed, "Value-0007");
    }
}
