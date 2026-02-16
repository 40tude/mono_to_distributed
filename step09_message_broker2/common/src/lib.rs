// Shared types and NATS subject constants for inter-service messaging

use serde::{Deserialize, Serialize};
use std::future::Future;

// --- NATS subjects (the "addresses" where messages are sent) ---

/// Subject for processing requests (service1 listens here).
pub const SUBJECT_PROCESS: &str = "service.process";

/// Subject for transformation requests (service2 listens here).
pub const SUBJECT_TRANSFORM: &str = "service.transform";

/// Subject for version queries to service1.
pub const SUBJECT_VERSION_SERVICE1: &str = "service.version.service1";

/// Subject for version queries to service2.
pub const SUBJECT_VERSION_SERVICE2: &str = "service.version.service2";

// --- NATS queue groups (load-balance instead of fan-out) ---

/// Queue group for process workers (service1 instances).
pub const QUEUE_PROCESS: &str = "process_workers";

/// Queue group for transform workers (service2 instances).
pub const QUEUE_TRANSFORM: &str = "transform_workers";

// --- Version request / response ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionRequest {
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionResponse {
    pub service_name: String,
    pub version: String,
    pub request_id: String,
}

// --- Request / Response types (identical to step06) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessRequest {
    pub value: i32,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessResponse {
    pub value: i32,
    pub processed: bool,
    pub request_id: String,
    /// `None` on success, `Some(message)` on domain error.
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformRequest {
    pub value: i32,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformResponse {
    pub original: i32,
    pub transformed: String,
    pub request_id: String,
    /// `None` on success, `Some(message)` on domain error.
    pub error: Option<String>,
}

// --- Messaging abstraction (DIP: depend on this trait, not on NATS directly) ---

/// Transport-layer abstraction over a message broker.
///
/// The publisher calls `request()` without knowing which broker implementation
/// sits behind it. Concrete adapters (e.g. `NatsMessaging`) live in the binary
/// crate that owns the connection.
pub trait Messaging {
    /// Sends `payload` to `subject` and waits for a single reply.
    fn request(
        &self,
        subject: &str,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>> + Send;
}
