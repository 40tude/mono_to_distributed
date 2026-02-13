// lib.rs

use serde::{Deserialize, Serialize};

// Common message types for inter-service communication

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessRequest {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessResponse {
    pub value: i32,
    pub processed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformRequest {
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformResponse {
    pub original: i32,
    pub transformed: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionResponse {
    pub service_name: String,
    pub version: String,
}

// Simple protocol for message exchange
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Process(ProcessRequest),
    ProcessResult(ProcessResponse),
    Transform(TransformRequest),
    TransformResult(TransformResponse),
    GetVersion,
    VersionResult(VersionResponse),
    Shutdown,
}

impl Message {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
