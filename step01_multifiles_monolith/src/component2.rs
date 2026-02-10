// component2.rs

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Component2Data {
    pub original: i32,
    pub transformed: String,
}

pub struct Component2;

impl Default for Component2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Component2 {
    pub fn new() -> Self {
        println!("[Component2] Initialized");
        Component2
    }

    pub fn transform(&self, input: i32) -> Component2Data {
        println!("[Component2] Transforming value: {}", input);
        let transformed = format!("Value-{:04}", input);

        Component2Data {
            original: input,
            transformed,
        }
    }

    pub fn analyze(&self, data: &Component2Data) -> String {
        println!("[Component2] Analyzing data: {:?}", data);
        format!("Analysis: {} maps to {}", data.original, data.transformed)
    }
}

