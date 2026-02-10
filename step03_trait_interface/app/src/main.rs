// main.rs
//
// Rust guideline compliant 2025-05-14

use component1_lib::Component1;
use component2_lib::Component2;
use traits::{Processor, Transformer};

fn main() {
    println!("\n\nPhase 02.5: Trait-Based Interface (shared contracts)\n");

    println!("Component1 version: {}", component1_lib::get_version());
    println!("Component2 version: {}", component2_lib::get_version());

    println!("\n--- Processing Pipeline ---");

    let input_value = 42;
    println!("Input value: {}", input_value);

    // Component 1: used through the Processor trait
    let comp1 = Component1::new();
    let data1 = comp1.process(input_value);
    let is_valid = comp1.validate(&data1);
    println!("Component1 result: {:?}, Valid: {}", data1, is_valid);

    // Component 2: used through the Transformer trait
    let comp2 = Component2::new();
    let data2 = comp2.transform(data1.value);
    let analysis = comp2.analyze(&data2);
    println!("Component2 result: {:?}", data2);
    println!("{}", analysis);

    // Demonstrate trait-based polymorphism: run_pipeline accepts any Processor + Transformer
    println!("\n--- Trait-Based Pipeline ---");
    run_pipeline(&comp1, &comp2, input_value);

    println!("\nExecution complete");
}

/// Run components through trait references.
/// This function knows NOTHING about Component1 or Component2 concrete types.
/// It only depends on the `traits` crate — not on any specific implementation.
fn run_pipeline(processor: &dyn Processor, transformer: &dyn Transformer, input: i32) {
    let processed = processor.process(input);
    println!("Pipeline processed: {:?}", processed);

    let transformed = transformer.transform(processed.value);
    println!("Pipeline transformed: {:?}", transformed);
    println!("Pipeline analysis: {}", transformer.analyze(&transformed));
}
