// lib.rs

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Debug)]
pub struct Component2Data {
    pub original: i32,
    pub transformed: String,
}

#[derive(Default)]
pub struct Component2;

impl Component2 {
    pub fn new() -> Self {
        println!("\t[Component2 DLL] Initialized");
        Component2
    }

    pub fn transform(&self, input: i32) -> Component2Data {
        println!("\t[Component2 DLL] Transforming value: {}", input);
        let transformed = format!("Value-{:04}", input);

        Component2Data {
            original: input,
            transformed,
        }
    }

    pub fn analyze(&self, data: &Component2Data) -> String {
        println!("\t[Component2 DLL] Analyzing data: {:?}", data);
        format!("Analysis: {} maps to {}", data.original, data.transformed)
    }
}

pub fn get_version() -> &'static str {
    // "1.0.0"
    env!("CARGO_PKG_VERSION")
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_transform() {
//         let comp = Component2::new();
//         let result = comp.transform(42);
//         assert_eq!(result.original, 42);
//         assert_eq!(result.transformed, "Value-0042");
//     }
// }
