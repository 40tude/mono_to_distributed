// main.rs

use common::{Message, TransformRequest, TransformResponse, VersionResponse};
use std::io::{self, BufRead, Write};

struct TransformationService;

fn main() {
    eprintln!("\t[Service2] Transformation Service");
    eprintln!("\t[Service2] Listening on STDIN for JSON messages...");

    let service = TransformationService::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
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
                    Ok(Message::GetVersion) => {
                        let message = Message::VersionResult(VersionResponse {
                            service_name: "service2".to_string(),
                            version: env!("CARGO_PKG_VERSION").to_string(),
                        });
                        writeln!(stdout, "{}", message.to_json()).unwrap();
                        stdout.flush().unwrap();
                    }
                    Ok(Message::Shutdown) => {
                        eprintln!("\t[Service2] Shutdown signal received");
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

impl TransformationService {
    fn new() -> Self {
        eprintln!("\t[Service2] Initialized - Ready to transform requests");
        TransformationService
    }

    fn transform(&self, request: TransformRequest) -> TransformResponse {
        eprintln!("\t[Service2] Input value: {}", request.value);

        // Transform the value to a formatted string
        let transformed = format!("Value-{:04}", request.value);

        TransformResponse {
            original: request.value,
            transformed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::TransformRequest;

    #[test]
    fn transform_formats_value() {
        let service = TransformationService::new();
        let request = TransformRequest { value: 42 };
        let response = service.transform(request);
        assert_eq!(response.original, 42);
        assert_eq!(response.transformed, "Value-0042");
    }

    #[test]
    fn transform_pads_small_value() {
        let service = TransformationService::new();
        let request = TransformRequest { value: 7 };
        let response = service.transform(request);
        assert_eq!(response.transformed, "Value-0007");
    }
}
