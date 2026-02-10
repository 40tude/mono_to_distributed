// main.rs

use common::{Message, TransformRequest, TransformResponse};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct TransformationService;

impl TransformationService {
    fn new() -> Self {
        eprintln!("[Service2] Initialized - Ready to transform requests");
        TransformationService
    }

    fn transform(&self, request: TransformRequest) -> TransformResponse {
        eprintln!("[Service2] Transforming request: {}", request.request_id);
        eprintln!("[Service2] Input value: {}", request.value);

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
    eprintln!("Service 2: Transformation Service");
    eprintln!("Listening on STDIN for JSON messages...");

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
                        eprintln!("[Service2] Shutdown signal received");
                        running.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {
                        eprintln!("[Service2] Unexpected message type");
                    }
                    Err(e) => {
                        eprintln!("[Service2] Failed to parse message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("[Service2] Error reading line: {}", e);
                break;
            }
        }
    }

    eprintln!("[Service2] Shutting down");
}
