// main.rs

use common::{Message, ProcessRequest, TransformRequest};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use uuid::Uuid;

struct ServiceHandle {
    process: Child,
    name: String,
}

impl ServiceHandle {
    fn new(name: &str, exe_path: &str) -> std::io::Result<Self> {
        println!("[Orchestrator] Starting service: {}", name);

        let process = Command::new(exe_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        Ok(ServiceHandle {
            process,
            name: name.to_string(),
        })
    }

    fn send_message(&mut self, message: &Message) -> std::io::Result<()> {
        if let Some(stdin) = self.process.stdin.as_mut() {
            let json = message.to_json();
            writeln!(stdin, "{}", json)?;
            stdin.flush()?;
        }
        Ok(())
    }

    fn read_response(&mut self) -> std::io::Result<String> {
        if let Some(stdout) = self.process.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            reader.read_line(&mut line)?;
            Ok(line)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "No stdout available",
            ))
        }
    }

    fn shutdown(mut self) -> std::io::Result<()> {
        println!("[Orchestrator] Shutting down service: {}", self.name);
        self.send_message(&Message::Shutdown)?;
        self.process.wait()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("\n\nPhase 05: Multi process\n");

    const HEADER: &str = "C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/048_mono_to_distributed/step05_multi_process";
    // const HEADER: &str = "target";

    // Paths to service executables (adjust based on your build)
    let service1_path = if cfg!(windows) {
        format!("{}{}", HEADER, String::from("/debug/service1.exe"))
    } else {
        String::from("/target/debug/service1")
    };

    let service2_path = if cfg!(windows) {
        format!("{}{}", HEADER, String::from("/debug/service2.exe"))
    } else {
        String::from("/target/debug/service2")
    };

    // Start services (these could be on different machines in a real distributed system)
    // Invert the 2 lines below
    let mut service1 = ServiceHandle::new("Service1", &service1_path)?;
    let mut service2 = ServiceHandle::new("Service2", &service2_path)?;

    println!("\n--- Processing Distributed Pipeline ---");

    // Generate a unique request ID
    let request_id = Uuid::new_v4().to_string();
    let input_value = 42;

    println!("[Orchestrator] Starting request: {}", request_id);
    println!("[Orchestrator] Input value: {}", input_value);

    // Step 1: Send to Service1 (processing)
    println!("[Orchestrator] Sending to Service1...");
    let process_request = ProcessRequest {
        value: input_value,
        request_id: request_id.clone(),
    };
    service1.send_message(&Message::Process(process_request))?;

    // Read response from Service1
    let response1_json = service1.read_response()?;
    let response1 =
        Message::from_json(&response1_json.trim()).expect("Failed to parse Service1 response");

    if let Message::ProcessResult(result) = response1 {
        println!(
            "[Orchestrator] Service1 result: value={}, processed={}",
            result.value, result.processed
        );

        // Step 2: Send to Service2 (transformation)
        println!("\n[Orchestrator] Sending to Service2...");
        let transform_request = TransformRequest {
            value: result.value,
            request_id: request_id.clone(),
        };
        service2.send_message(&Message::Transform(transform_request))?;

        // Read response from Service2
        let response2_json = service2.read_response()?;
        let response2 =
            Message::from_json(&response2_json.trim()).expect("Failed to parse Service2 response");

        if let Message::TransformResult(result) = response2 {
            println!(
                "[Orchestrator] Service2 result: original={}, transformed={}",
                result.original, result.transformed
            );

            println!("\n[Orchestrator] Pipeline completed successfully!");
            println!("[Orchestrator] Final result: {}", result.transformed);
        }
    }

    // Cleanup
    println!("\n[Orchestrator] Cleaning up services...");
    service1.shutdown()?;
    service2.shutdown()?;

    println!("[Orchestrator] All services shut down");
    println!("\n[Orchestrator] Execution complete");

    Ok(())
}
