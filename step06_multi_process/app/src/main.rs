// main.rs

use common::{Message, ProcessRequest, TransformRequest, VersionResponse};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

struct ServiceHandle {
    process: Child,
    name: String,
}

fn main() -> std::io::Result<()> {
    // Build path to services
    #[cfg(feature = "40tude")]
    const HEADER: &str = "C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/048_mono_to_distributed/step06_multi_process";
    #[cfg(not(feature = "40tude"))]
    const HEADER: &str = "target";

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

    println!("\n\nPhase 06: Multi process\n");

    let mut service1 = ServiceHandle::new("Service1", &service1_path)?;
    let mut service2 = ServiceHandle::new("Service2", &service2_path)?;

    // Query service versions
    service1.send_message(&Message::GetVersion)?;
    let v1_json = service1.read_response()?;
    if let Ok(Message::VersionResult(VersionResponse {
        service_name,
        version,
    })) = Message::from_json(v1_json.trim())
    {
        println!("[App] {} version: {}", service_name, version);
    }

    service2.send_message(&Message::GetVersion)?;
    let v2_json = service2.read_response()?;
    if let Ok(Message::VersionResult(VersionResponse {
        service_name,
        version,
    })) = Message::from_json(v2_json.trim())
    {
        println!("[App] {} version: {}", service_name, version);
    }

    println!("\n--- Processing Distributed Pipeline ---");

    let input_value = 42;
    println!("[App] Input value: {}", input_value);

    // Send to Service1: processing
    println!("[App] Sending to Service1...");
    let process_request = ProcessRequest { value: input_value };
    service1.send_message(&Message::Process(process_request))?;

    // Read response from Service1
    let response1_json = service1.read_response()?;
    let response1 =
        Message::from_json(&response1_json.trim()).expect("Failed to parse Service1 response");

    if let Message::ProcessResult(result) = response1 {
        println!(
            "[App] Service1 result: value={}, processed={}",
            result.value, result.processed
        );

        // Send to Service2: transformation
        println!("\n[App] Sending to Service2...");
        let transform_request = TransformRequest {
            value: result.value,
        };
        service2.send_message(&Message::Transform(transform_request))?;

        // Read response from Service2
        let response2_json = service2.read_response()?;
        let response2 =
            Message::from_json(&response2_json.trim()).expect("Failed to parse Service2 response");

        if let Message::TransformResult(result) = response2 {
            println!(
                "[App] Service2 result: original={}, transformed={}",
                result.original, result.transformed
            );

            println!("\n[App] Pipeline completed successfully!");
            println!("[App] Final result: {}", result.transformed);
        }
    }

    // Cleanup
    println!("\n[App] Cleaning up services...");
    service1.shutdown()?;
    service2.shutdown()?;

    println!("[App] All services shut down");
    println!("\n[App] Execution complete");

    Ok(())
}

impl ServiceHandle {
    fn new(name: &str, exe_path: &str) -> std::io::Result<Self> {
        println!("[App] Starting service: {}", name);

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
        println!("[App] Shutting down service: {}", self.name);
        self.send_message(&Message::Shutdown)?;
        self.process.wait()?;
        Ok(())
    }
}
