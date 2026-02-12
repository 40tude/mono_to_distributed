// main.rs

// Rust guideline compliant 2025-05-01
//
// Host application: dynamically loads, uses, and unloads component DLL plugins.
// Demonstrates explicit load -> call -> unload cycle with libloading.

use std::env;
use std::error::Error;
use std::ffi::CStr;

use libloading::{Library, Symbol};
use plugin_interface::{
    PLUGIN_CREATE_SYMBOL, PLUGIN_DESTROY_SYMBOL, PLUGIN_VERSION_SYMBOL, PluginVersion,
    ProcessPluginCreate, ProcessPluginDestroy, TransformPluginCreate, TransformPluginDestroy,
};

#[cfg(target_os = "windows")]
const PLUGIN_EXT: &str = "dll";

#[cfg(target_os = "linux")]
const PLUGIN_EXT: &str = "so";

#[cfg(target_os = "macos")]
const PLUGIN_EXT: &str = "dylib";

/// Resolve DLL path relative to the running executable.
fn plugin_path(name: &str) -> String {
    let exe = env::current_exe().expect("cannot determine exe path");
    let dir = exe.parent().expect("cannot determine exe directory");
    dir.join(format!("{name}.{PLUGIN_EXT}"))
        .to_string_lossy()
        .to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n\nPhase 05: Modular Application with Dynamic Plugins (runtime DLL load/unload)\n");

    println!("--- Processing Pipeline ---");

    let input_value = 42;
    println!("Input value: {input_value}\n");

    // Component 1 processing (from DLL)
    let comp1_path = plugin_path("component1");
    println!("Loading component1 DLL from: {comp1_path}");

    let process_result = {
        // SAFETY: we load a known DLL built from component1_lib crate
        let lib = unsafe { Library::new(&comp1_path)? };
        println!("Component1 DLL loaded");

        // SAFETY: symbol exported by component1_lib, returns pointer to static C string
        let version: Symbol<PluginVersion> = unsafe { lib.get(PLUGIN_VERSION_SYMBOL)? };
        // SAFETY: returned pointer is a valid null-terminated static string
        let ver = unsafe { CStr::from_ptr(version()) }.to_str()?;
        println!("Component1 version: {ver}");

        // SAFETY: symbol exported by component1_lib with matching signature
        let create: Symbol<ProcessPluginCreate> = unsafe { lib.get(PLUGIN_CREATE_SYMBOL)? };
        // SAFETY: `create` returns a valid heap-allocated trait object
        let plugin_ptr = unsafe { create() };
        // SAFETY: pointer is valid and non-null, created just above
        let plugin = unsafe { &*plugin_ptr };

        let result = plugin.process(input_value);
        let is_valid = plugin.validate(&result);
        println!("Process result: {result:?}, Valid: {is_valid}");

        // SAFETY: symbol exported by component1_lib with matching signature
        let destroy: Symbol<ProcessPluginDestroy> = unsafe { lib.get(PLUGIN_DESTROY_SYMBOL)? };
        // SAFETY: `plugin_ptr` was created by `create()` and not yet destroyed
        unsafe { destroy(plugin_ptr) };

        // `lib` drops here => DLL unloaded
        println!("Unloading Component1 DLL...\n");
        result
    };

    // Component 2 processing (from DLL)
    let comp2_path = plugin_path("component2");
    println!("Loading component2 DLL from: {comp2_path}");

    let _transform_result = {
        // SAFETY: we load a known DLL built from component2_lib crate
        let lib = unsafe { Library::new(&comp2_path)? };
        println!("Component2 DLL loaded");

        // SAFETY: symbol exported by component2_lib, returns pointer to static C string
        let version: Symbol<PluginVersion> = unsafe { lib.get(PLUGIN_VERSION_SYMBOL)? };
        // SAFETY: returned pointer is a valid null-terminated static string
        let ver = unsafe { CStr::from_ptr(version()) }.to_str()?;
        println!("Component2 version: {ver}");

        // SAFETY: symbol exported by component2_lib with matching signature
        let create: Symbol<TransformPluginCreate> = unsafe { lib.get(PLUGIN_CREATE_SYMBOL)? };
        // SAFETY: `create` returns a valid heap-allocated trait object
        let plugin_ptr = unsafe { create() };
        // SAFETY: pointer is valid and non-null, created just above
        let plugin = unsafe { &*plugin_ptr };

        let result = plugin.transform(process_result.value);
        let analysis = plugin.analyze(&result);
        println!("Transform result: {result:?}");
        println!("{analysis}");

        // SAFETY: symbol exported by component2_lib with matching signature
        let destroy: Symbol<TransformPluginDestroy> = unsafe { lib.get(PLUGIN_DESTROY_SYMBOL)? };
        // SAFETY: `plugin_ptr` was created by `create()` and not yet destroyed
        unsafe { destroy(plugin_ptr) };

        // `lib` drops here => DLL unloaded
        println!("Unloading component2 DLL...\n");
        result
    };

    // println!("Original value  : {}", input_value);
    // println!("Processed value : {}", process_result.value);
    // println!("Transformed     : {}", transform_result.transformed);

    println!("Execution complete");

    Ok(())
}
