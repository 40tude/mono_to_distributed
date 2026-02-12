// main.rs

use component1::Component1;
use component2::Component2;
use traits::{Processor, Transformer};

fn main() {
    println!("\n\nPhase 04: Modular Application with Plugins (1 exe + 2 dll)\n");

    println!("Component1 version: {}", component1::get_version());
    println!("Component2 version: {}", component2::get_version());

    let input_value = 42;
    let comp1 = Component1::new();
    let comp2 = Component2::new();
    run_pipeline(&comp1, &comp2, input_value);

    println!("\nExecution complete");
}

/// Run components through trait references.
/// This function knows NOTHING about Component1 or Component2 concrete types.
/// It only depends on the `traits` crate — not on any specific implementation.
fn run_pipeline(processor: &dyn Processor, transformer: &dyn Transformer, input: i32) {
    println!("\n--- Processing Pipeline ---");

    println!("Input value: {}", input);

    let data1 = processor.process(input);
    let is_valid = processor.validate(&data1);
    println!("Component1 result: {:?}, Valid: {}", data1, is_valid);

    let data2 = transformer.transform(data1.value);
    let analysis = transformer.analyze(&data2);
    println!("Component2 result: {:?}", data2);
    println!("{}", analysis);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_pipeline() {
        let comp1 = Component1::new();
        let comp2 = Component2::new();
        // run_pipeline accepts any &dyn Processor + &dyn Transformer
        // Here we pass the concrete types but the function only sees the traits
        run_pipeline(&comp1, &comp2, 42);
    }
}
