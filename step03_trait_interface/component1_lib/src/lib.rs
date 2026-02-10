// Component1 — implements the Processor trait from the shared `traits` crate
//
// Rust guideline compliant 2025-05-14

use traits::{ProcessResult, Processor};

#[derive(Default)]
pub struct Component1;

impl Component1 {
    pub fn new() -> Self {
        println!("\t[Component1 Lib] Initialized");
        Component1
    }
}

impl Processor for Component1 {
    fn process(&self, input: i32) -> ProcessResult {
        println!("\t[Component1 Lib] Processing value: {}", input);
        let result = input * 2;

        ProcessResult {
            value: result,
            processed: true,
        }
    }

    fn validate(&self, data: &ProcessResult) -> bool {
        println!("\t[Component1 Lib] Validating data: {:?}", data);
        data.processed && data.value > 0
    }
}

/// Public API for version info.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let comp = Component1::new();
        let result = comp.process(21);
        assert_eq!(result.value, 42);
        assert!(result.processed);
    }
}
