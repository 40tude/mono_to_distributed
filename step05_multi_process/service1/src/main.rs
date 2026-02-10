// main.rs

use common::{Message, ProcessRequest, ProcessResponse};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct ProcessingService;

impl ProcessingService {
    fn new() -> Self {
        eprintln!("[Service1] Initialized - Ready to process requests");
        ProcessingService
    }

    fn process(&self, request: ProcessRequest) -> ProcessResponse {
        eprintln!("[Service1] Processing request: {}", request.request_id);
        eprintln!("[Service1] Input value: {}", request.value);

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
    eprintln!("Service 1: Processing Service");
    eprintln!("Listening on STDIN for JSON messages...\n");

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
                        eprintln!("[Service1] Shutdown signal received");
                        running.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {
                        eprintln!("[Service1] Unexpected message type");
                    }
                    Err(e) => {
                        eprintln!("[Service1] Failed to parse message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("[Service1] Error reading line: {}", e);
                break;
            }
        }
    }

    eprintln!("[Service1] Shutting down");
}

