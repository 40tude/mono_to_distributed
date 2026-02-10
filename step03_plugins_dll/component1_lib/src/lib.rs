// lib.rs

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Debug)]
pub struct Component1Data {
    pub value: i32,
    pub processed: bool,
}

#[derive(Default)]
pub struct Component1;

impl Component1 {
    pub fn new() -> Self {
        println!("[Component1 DLL] Initialized");
        Component1
    }

    pub fn process(&self, input: i32) -> Component1Data {
        println!("[Component1 DLL] Processing value: {}", input);
        let result = input * 2;

        Component1Data {
            value: result,
            processed: true,
        }
    }

    pub fn validate(&self, data: &Component1Data) -> bool {
        println!("[Component1 DLL] Validating data: {:?}", data);
        data.processed && data.value > 0
    }
}

// Public API for version info
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

