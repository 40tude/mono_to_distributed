// main.rs

use common::{Message, ProcessRequest, ProcessResponse};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct ProcessingService;

impl ProcessingService {
    fn new() -> Self {
        eprintln!("\t[Service1] Initialized - Ready to process requests");
        ProcessingService
    }

    fn process(&self, request: ProcessRequest) -> ProcessResponse {
        eprintln!("\t[Service1] Processing request: {}", request.request_id);
        eprintln!("\t[Service1] Input value: {}", request.value);

        // Simulate some processing (multiply by 2)
        let result = request.value * 2;

        ProcessResponse {
            value: result,
            processed: true,
            request_id: request.request_id,
        }
    }
}

fn main() {
    eprintln!("\t[Service1] Processing Service");
    eprintln!("\t[Service1] Listening on STDIN for JSON messages...");

    let service = ProcessingService::new();
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
                    Ok(Message::Process(request)) => {
                        let response = service.process(request);
                        let message = Message::ProcessResult(response);
                        let json = message.to_json();

                        writeln!(stdout, "{}", json).unwrap();
                        stdout.flush().unwrap();
                    }
                    Ok(Message::Shutdown) => {
                        eprintln!("\t[Service1] Shutdown signal received");
                        running.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {
                        eprintln!("\t[Service1] Unexpected message type");
                    }
                    Err(e) => {
                        eprintln!("\t[Service1] Failed to parse message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("\t[Service1] Error reading line: {}", e);
                break;
            }
        }
    }

    eprintln!("\t[Service1] Shutting down");
}

#[cfg(test)]
mod test {
    use super::*;
    use common::ProcessRequest;

    #[test]
    fn negative_value_stays_negative() {
        let service = ProcessingService::new();
        let request = ProcessRequest {
            value: -18,
            request_id: "test-1".to_string(),
        };
        let response = service.process(request);
        assert!(response.processed);
        assert_eq!(response.value, -36);
    }

    #[test]
    fn process_doubles_value() {
        let service = ProcessingService::new();
        let request = ProcessRequest {
            value: 21,
            request_id: "test-2".to_string(),
        };
        let response = service.process(request);
        assert_eq!(response.value, 42);
        assert!(response.processed);
    }
}
