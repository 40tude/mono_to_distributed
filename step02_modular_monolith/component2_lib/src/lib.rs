// lib.rs

#[derive(Debug)]
pub struct Component2Data {
    pub original: i32,
    pub transformed: String,
}

#[derive(Default)]
pub struct Component2;

impl Component2 {
    pub fn new() -> Self {
        println!("\t[Component2 Lib] Initialized");
        Component2
    }

    pub fn transform(&self, input: i32) -> Component2Data {
        println!("\t[Component2 Lib] Transforming value: {}", input);
        let transformed = format!("Value-{:04}", input);

        Component2Data {
            original: input,
            transformed,
        }
    }

    pub fn analyze(&self, data: &Component2Data) -> String {
        println!("\t[Component2 Lib] Analyzing data: {:?}", data);
        format!("Analysis: {} maps to {}", data.original, data.transformed)
    }
}

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
