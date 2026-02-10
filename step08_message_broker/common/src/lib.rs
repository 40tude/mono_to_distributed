// Shared types and NATS subject constants for inter-service messaging
//
// Rust guideline compliant 2025-05-14

use serde::{Deserialize, Serialize};

// --- NATS subjects (the "addresses" where messages are sent) ---

/// Subject for processing requests (service1 listens here).
pub const SUBJECT_PROCESS: &str = "service.process";

/// Subject for transformation requests (service2 listens here).
pub const SUBJECT_TRANSFORM: &str = "service.transform";

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
}
