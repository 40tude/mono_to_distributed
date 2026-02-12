// lib.rs

// Shared plugin interface: traits, data structs, and FFI symbol definitions.
// Both plugin DLLs and the host app depend on this crate.

/// Result of processing by component1 plugin.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub value: i32,
    pub processed: bool,
}

/// Result of transformation by component2 plugin.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TransformResult {
    pub original: i32,
    pub transformed: String,
}

/// Trait implemented by component1-style plugins.
pub trait ProcessPlugin: Send + Sync {
    fn process(&self, input: i32) -> ProcessResult;
    fn validate(&self, data: &ProcessResult) -> bool;
}

/// Trait implemented by component2-style plugins.
pub trait TransformPlugin: Send + Sync {
    fn transform(&self, input: i32) -> TransformResult;
    fn analyze(&self, data: &TransformResult) -> String;
}

// FFI function pointer types for dynamic loading

#[expect(
    improper_ctypes_definitions,
    reason = "trait object pointers cross DLL boundary by design"
)]
pub type ProcessPluginCreate = unsafe extern "C" fn() -> *mut dyn ProcessPlugin;

#[expect(
    improper_ctypes_definitions,
    reason = "trait object pointers cross DLL boundary by design"
)]
pub type ProcessPluginDestroy = unsafe extern "C" fn(*mut dyn ProcessPlugin);

#[expect(
    improper_ctypes_definitions,
    reason = "trait object pointers cross DLL boundary by design"
)]
pub type TransformPluginCreate = unsafe extern "C" fn() -> *mut dyn TransformPlugin;

#[expect(
    improper_ctypes_definitions,
    reason = "trait object pointers cross DLL boundary by design"
)]
pub type TransformPluginDestroy = unsafe extern "C" fn(*mut dyn TransformPlugin);

/// Returns a pointer to a null-terminated version string owned by the DLL.
pub type PluginVersion = unsafe extern "C" fn() -> *const std::ffi::c_char;

// Symbol names exported by each plugin DLL
pub const PLUGIN_CREATE_SYMBOL: &[u8] = b"_plugin_create";
pub const PLUGIN_DESTROY_SYMBOL: &[u8] = b"_plugin_destroy";
pub const PLUGIN_VERSION_SYMBOL: &[u8] = b"_plugin_version";
