// Rust guideline compliant 2025-05-01
//
// Component1 DLL plugin: implements ProcessPlugin trait from plugin_interface.
// Exports extern "C" factory functions for dynamic loading.

use std::ffi::c_char;

use plugin_interface::{ProcessPlugin, ProcessResult};

#[derive(Debug, Default)]
struct Component1;

impl Component1 {
    fn new() -> Self {
        println!("[Component1 DLL] Initialized");
        Self
    }
}

impl ProcessPlugin for Component1 {
    fn process(&self, input: i32) -> ProcessResult {
        println!("[Component1 DLL] Processing value: {input}");
        let result = input * 2;
        ProcessResult {
            value: result,
            processed: true,
        }
    }

    fn validate(&self, data: &ProcessResult) -> bool {
        println!("[Component1 DLL] Validating data: {data:?}");
        data.processed && data.value > 0
    }
}

/// # Safety
/// Returns a heap-allocated trait object pointer. Caller must pair with `_plugin_destroy`.
#[unsafe(no_mangle)]
#[expect(improper_ctypes_definitions, reason = "trait object pointer crosses DLL boundary by design")]
pub extern "C" fn _plugin_create() -> *mut dyn ProcessPlugin {
    let plugin = Component1::new();
    Box::into_raw(Box::new(plugin))
}

/// # Safety
/// `plugin` must be a pointer previously returned by `_plugin_create` and not yet destroyed.
#[unsafe(no_mangle)]
#[expect(improper_ctypes_definitions, reason = "trait object pointer crosses DLL boundary by design")]
pub extern "C" fn _plugin_destroy(plugin: *mut dyn ProcessPlugin) {
    println!("[Component1 DLL] Destroying plugin instance");
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
    // Null-terminated at compile time via concat!
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast::<c_char>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let comp = Component1::new();
        let result = comp.process(21);
        assert_eq!(result.value, 42);
        assert!(result.processed);
    }

    #[test]
    fn test_validate() {
        let comp = Component1::new();
        let result = comp.process(21);
        assert!(comp.validate(&result));
    }
}
