// Component2 — implements the Transformer trait from the shared `traits` crate
//
// Rust guideline compliant 2025-05-14

use traits::{TransformResult, Transformer};

#[derive(Default)]
pub struct Component2;

impl Component2 {
    pub fn new() -> Self {
        println!("\t[Component2 Lib] Initialized");
        Component2
    }
}

impl Transformer for Component2 {
    fn transform(&self, input: i32) -> TransformResult {
        println!("\t[Component2 Lib] Transforming value: {}", input);
        let transformed = format!("Value-{:04}", input);

        TransformResult {
            original: input,
            transformed,
        }
    }

    fn analyze(&self, data: &TransformResult) -> String {
        println!("\t[Component2 Lib] Analyzing data: {:?}", data);
        format!("Analysis: {} maps to {}", data.original, data.transformed)
    }
}

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let comp = Component2::new();
        let result = comp.transform(42);
        assert_eq!(result.original, 42);
        assert_eq!(result.transformed, "Value-0042");
    }
}
