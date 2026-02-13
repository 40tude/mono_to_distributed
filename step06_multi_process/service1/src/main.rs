// main.rs

use common::{Message, ProcessRequest, ProcessResponse, VersionResponse};
use std::io::{self, BufRead, Write};

struct ProcessingService;

fn main() {
    eprintln!("\t[Service1] Processing Service");
    eprintln!("\t[Service1] Listening on STDIN for JSON messages...");

    let service = ProcessingService::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
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
                    Ok(Message::GetVersion) => {
                        let message = Message::VersionResult(VersionResponse {
                            service_name: "service1".to_string(),
                            version: env!("CARGO_PKG_VERSION").to_string(),
                        });
                        writeln!(stdout, "{}", message.to_json()).unwrap();
                        stdout.flush().unwrap();
                    }
                    Ok(Message::Shutdown) => {
                        eprintln!("\t[Service1] Shutdown signal received");
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

impl ProcessingService {
    fn new() -> Self {
        eprintln!("\t[Service1] Initialized - Ready to process requests");
        ProcessingService
    }

    fn process(&self, request: ProcessRequest) -> ProcessResponse {
        eprintln!("\t[Service1] Input value: {}", request.value);

        // Simulate some processing (multiply by 2)
        let result = request.value * 2;

        ProcessResponse {
            value: result,
            processed: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::ProcessRequest;

    #[test]
    fn negative_value_stays_negative() {
        let service = ProcessingService::new();
        let request = ProcessRequest { value: -18 };
        let response = service.process(request);
        assert!(response.processed);
        assert_eq!(response.value, -36);
    }

    #[test]
    fn process_doubles_value() {
        let service = ProcessingService::new();
        let request = ProcessRequest { value: 21 };
        let response = service.process(request);
        assert_eq!(response.value, 42);
        assert!(response.processed);
    }
}
