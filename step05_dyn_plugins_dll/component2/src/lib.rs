// lib.rs

// Rust guideline compliant 2025-05-01
//
// Component2 DLL plugin: implements TransformPlugin trait from plugin_interface.
// Exports extern "C" factory functions for dynamic loading.

use std::ffi::c_char;

use plugin_interface::{TransformPlugin, TransformResult};

#[derive(Debug, Default)]
struct Component2;

impl Component2 {
    fn new() -> Self {
        println!("\t[Component2 DLL] Initialized");
        Self
    }
}

impl TransformPlugin for Component2 {
    fn transform(&self, input: i32) -> TransformResult {
        println!("\t[Component2 DLL] Transforming value: {input}");
        let transformed = format!("Value-{input:04}");
        TransformResult {
            original: input,
            transformed,
        }
    }

    fn analyze(&self, data: &TransformResult) -> String {
        println!("\t[Component2 DLL] Analyzing data: {data:?}");
        format!("Analysis: {} maps to {}", data.original, data.transformed)
    }
}

/// # Safety
/// Returns a heap-allocated trait object pointer. Caller must pair with `_plugin_destroy`.
#[unsafe(no_mangle)]
#[expect(
    improper_ctypes_definitions,
    reason = "trait object pointer crosses DLL boundary by design"
)]
pub extern "C" fn _plugin_create() -> *mut dyn TransformPlugin {
    let plugin = Component2::new();
    Box::into_raw(Box::new(plugin))
}

/// # Safety
/// `plugin` must be a pointer previously returned by `_plugin_create` and not yet destroyed.
#[unsafe(no_mangle)]
#[expect(
    improper_ctypes_definitions,
    reason = "trait object pointer crosses DLL boundary by design"
)]
pub extern "C" fn _plugin_destroy(plugin: *mut dyn TransformPlugin) {
    println!("[Component2 DLL] Destroying plugin instance");
    if !plugin.is_null() {
        // SAFETY: pointer was created by `_plugin_create` via `Box::into_raw`
        unsafe {
            drop(Box::from_raw(plugin));
        }
    }
}

/// Returns the crate version as a null-terminated C string.
#[unsafe(no_mangle)]
pub extern "C" fn _plugin_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0")
        .as_ptr()
        .cast::<c_char>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let comp = Component2::new();
        let result = comp.transform(42);
        assert_eq!(result.original, 42);
        assert_eq!(result.transformed, "Value-0042");
    }

    #[test]
    fn test_analyze() {
        let comp = Component2::new();
        let result = comp.transform(42);
        let analysis = comp.analyze(&result);
        assert!(analysis.contains("42"));
        assert!(analysis.contains("Value-0042"));
    }
}

