// lib.rs

use serde::{Deserialize, Serialize};

// Shared request/response types for inter-service HTTP communication

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

