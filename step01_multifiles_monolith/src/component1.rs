// component1.rs

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Component1Data {
    pub value: i32,
    pub processed: bool,
}

pub struct Component1;

impl Default for Component1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Component1 {
    pub fn new() -> Self {
        println!("[Component1] Initialized");
        Component1
    }

    pub fn process(&self, input: i32) -> Component1Data {
        println!("[Component1] Processing value: {}", input);
        let result = input * 2; // Simple processing: multiply by 2

        Component1Data {
            value: result,
            processed: true,
        }
    }

    pub fn validate(&self, data: &Component1Data) -> bool {
        println!("[Component1] Validating data: {:?}", data);
        data.processed && data.value > 0
    }
}

