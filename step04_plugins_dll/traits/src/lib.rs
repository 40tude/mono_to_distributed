// Shared trait definitions (interface contracts)
//
// This crate defines the contracts that components must implement.
// It contains NO business logic — only trait signatures and shared data types.
// Components depend on this crate, but this crate depends on nothing.

/// Result of a processing operation.
#[derive(Debug)]
pub struct ProcessResult {
    pub value: i32,
    pub processed: bool,
}

/// Result of a transformation operation.
#[derive(Debug)]
pub struct TransformResult {
    pub original: i32,
    pub transformed: String,
}

/// Contract for components that process numeric input.
pub trait Processor {
    fn process(&self, input: i32) -> ProcessResult;
    fn validate(&self, data: &ProcessResult) -> bool;
}

/// Contract for components that transform numeric input into a string.
pub trait Transformer {
    fn transform(&self, input: i32) -> TransformResult;
    fn analyze(&self, data: &TransformResult) -> String;
}
