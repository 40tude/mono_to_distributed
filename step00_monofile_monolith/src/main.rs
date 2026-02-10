// main.rs

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

fn main() {
    println!("\n\nPhase 00: Mono File Monolith Application\n");

    // Initialize all components
    let comp1 = Component1::new();
    let comp2 = Component2::new();

    println!("\n--- Processing Pipeline ---");

    // Simulate a processing pipeline
    let input_value = 42;
    println!("Input value: {}", input_value);

    // Component 1 processing
    let data1 = comp1.process(input_value);
    let is_valid = comp1.validate(&data1);
    println!("Component1 result: {:?}, Valid: {}", data1, is_valid);

    // Component 2 processing (using Component 1's output)
    let data2 = comp2.transform(data1.value);
    let analysis = comp2.analyze(&data2);
    println!("Component2 result: {:?}", data2);
    println!("{}", analysis);

    println!("\nExecution complete");
}
