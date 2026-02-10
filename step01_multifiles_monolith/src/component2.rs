// component2.rs

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
        println!("\t[Component2] Initialized");
        Component2
    }

    pub fn transform(&self, input: i32) -> Component2Data {
        println!("\t[Component2] Transforming value: {}", input);
        let transformed = format!("Value-{:04}", input);

        Component2Data {
            original: input,
            transformed,
        }
    }

    pub fn analyze(&self, data: &Component2Data) -> String {
        println!("\t[Component2] Analyzing data: {:?}", data);
        format!("Analysis: {} maps to {}", data.original, data.transformed)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_transform() {
        let comp = Component2::new();
        let result = comp.transform(42);
        assert_eq!(result.original, 42);
        assert_eq!(result.transformed, "Value-0042");
    }
}
