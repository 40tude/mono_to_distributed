// main.rs

use component1_dll::Component1;
use component2_dll::Component2;

fn main() {
    println!("\n\nPhase 03: Modular Application with Plugins (1 exe + 2 dll)\n");

    println!("Component1 version: {}", component1_dll::get_version());
    println!("Component2 version: {}", component2_dll::get_version());

    // Initialize components from separate DLLs

    println!("\n--- Processing Pipeline ---");

    let input_value = 42;
    println!("Input value: {}", input_value);

    // Component 1 processing (from DLL)
    let comp1 = Component1::new();
    let data1 = comp1.process(input_value);
    let is_valid = comp1.validate(&data1);
    println!("Component1 result: {:?}, Valid: {}", data1, is_valid);

    // Component 2 processing (from DLL)
    let comp2 = Component2::new();
    let data2 = comp2.transform(data1.value);
    let analysis = comp2.analyze(&data2);
    println!("Component2 result: {:?}", data2);
    println!("{}", analysis);

    println!("\nExecution complete");
}
