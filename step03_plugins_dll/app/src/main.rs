// main.rs

use component1_lib::Component1;
use component2_lib::Component2;

fn main() {
    println!("\n\nPhase 03: Plugins (1 exe + 2 dll)\n");

    println!("Component1 version: {}", component1_lib::get_version());
    println!("Component2 version: {}", component2_lib::get_version());

    // Initialize components from separate DLLs
    let comp1 = Component1::new();
    let comp2 = Component2::new();

    println!("\n--- Processing Pipeline ---");

    let input_value = 42;
    println!("Input value: {}", input_value);

    // Component 1 processing (from DLL)
    let data1 = comp1.process(input_value);
    let is_valid = comp1.validate(&data1);
    println!("Component1 result: {:?}, Valid: {}", data1, is_valid);

    // Component 2 processing (from DLL)
    let data2 = comp2.transform(data1.value);
    let analysis = comp2.analyze(&data2);
    println!("Component2 result: {:?}", data2);
    println!("{}", analysis);

    println!("\nExecution complete");
}
