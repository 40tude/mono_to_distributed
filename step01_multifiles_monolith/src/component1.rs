// component1.rs

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
        println!("\t[Component1] Initialized");
        Component1
    }

    pub fn process(&self, input: i32) -> Component1Data {
        println!("\t[Component1] Processing value: {}", input);
        let result = input * 2;

        Component1Data {
            value: result,
            processed: true,
        }
    }

    pub fn validate(&self, data: &Component1Data) -> bool {
        println!("\t[Component1] Validating data: {:?}", data);
        data.processed && data.value > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negative_number_are_invalid() {
        let comp1 = Component1::new();
        let data1 = comp1.process(-18);
        let is_valid = comp1.validate(&data1);
        assert!(!is_valid);
    }
}
