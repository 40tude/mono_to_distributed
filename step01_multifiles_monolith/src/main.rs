// main.rs

mod component1;
mod component2;

use component1::Component1;
use component2::Component2;

fn main() {
    println!("\n\nPhase 01: Multi Files Monolith Application\n");

    println!("\n--- Processing Pipeline ---");

    // Simulate a processing pipeline
    let input_value = 42;
    println!("Input value: {}", input_value);

    // Component 1 processing
    let comp1 = Component1::new();
    let data1 = comp1.process(input_value);
    let is_valid = comp1.validate(&data1);
    println!("Component1 result: {:?}, Valid: {}", data1, is_valid);

    // Component 2 processing (using Component 1's output)
    let comp2 = Component2::new();
    let data2 = comp2.transform(data1.value);
    let analysis = comp2.analyze(&data2);
    println!("Component2 result: {:?}", data2);
    println!("{}", analysis);

    println!("\nExecution complete");
}
