# Step 03 — Plugins DLL (cdylib + rlib)

## Build & Run

```powershell
cargo build                    # build all workspace members
cargo run
cargo test
```

## What Changed from Step 02

Only one line changed per component Cargo.toml — we uncommented `crate-type`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

This tells Cargo to produce **two** artifacts for each component:

| Artifact | Format | Purpose |
|----------|--------|---------|
| `libcomponent1_dll.rlib` | Rust static library | Used by `cargo build` to link into `app.exe` |
| `component1_dll.dll` | Windows dynamic library | Available for external consumers (C, C++, other Rust binaries...) |

The app code (`main.rs`) is **identical** to step02. No code change at all.

## The Key Insight: The DLLs Are NOT Used

This is the most important thing to understand in this step.

When you run `cargo build`, Cargo produces both `.rlib` and `.dll` files. But when it
links `app.exe`, **it uses the `.rlib` (static library), not the `.dll`**.

You can verify this yourself:

```powershell
# Rename the DLLs — the exe still works!
cd target/debug
ren component1_dll.dll component1_dll.dll.bak
ren component2_dll.dll component2_dll.dll.bak
.\app.exe                      # runs perfectly
```

The exe is **self-contained**. The component code is baked into it, just like in step02.
The DLLs sit in the build folder, unused.

### Why does Cargo use rlib instead of the DLL?

Because `app/Cargo.toml` says:

```toml
[dependencies]
component1_dll = { path = "../component1_dll" }
```

This is a **Rust-to-Rust** dependency. Cargo always prefers `rlib` for these because:

1. Rust's ABI is unstable — it can change between compiler versions
2. Static linking is simpler and faster
3. `rlib` preserves full type info, generics, and monomorphization

The `cdylib` output exists for **foreign consumers** (C, Python, etc.) that would load the
DLL via FFI. Rust itself never uses it through a `path` dependency.

## So Why Does This Step Exist?

This step is a **preparation step**. It demonstrates that:

1. You can tell Cargo to *produce* DLLs alongside the static library
2. The DLLs are real, loadable dynamic libraries (step04 will use them)
3. The app code doesn't change at all — the transition is invisible

Think of it as: "your components are now *capable* of being DLLs, even though nobody is
loading them dynamically yet."

## Build Artifacts

After `cargo build`, the `target/debug/` folder contains:

```
app.exe                  (155 KB)  ← standalone, contains all component code
component1_dll.dll       (114 KB)  ← produced but NOT used by app.exe
component1_dll.dll.lib   (  2 KB)  ← import library (for C/C++ linkers)
component1_dll.dll.exp   (  1 KB)  ← export table
libcomponent1_dll.rlib   ( 10 KB)  ← what Cargo actually links into app.exe
component2_dll.dll       (114 KB)  ← produced but NOT used
component2_dll.dll.lib   (  2 KB)
component2_dll.dll.exp   (  1 KB)
libcomponent2_dll.rlib   ( 10 KB)
```

## Address Space Reminder

Since the DLLs are not actually loaded in this step, the address space question is moot
here. But since step04 *will* load them, here is the important concept:

### DLLs Share the Process Address Space

Contrary to a common misconception, **a DLL does NOT have its own separate address space**.

When a DLL is loaded into a process (whether at startup or at runtime), its code and data
are mapped into the **same virtual address space** as the exe. Everything — the exe's code,
the DLL's code, the heap, the stack — lives in one flat 64-bit address space.

```
┌─────────────────────────────────────────────────────┐
│              Process Virtual Address Space           │
│                                                     │
│  0x00007FF6_00000000   app.exe code + data           │
│  0x00007FFB_10000000   component1_dll.dll            │
│  0x00007FFB_20000000   component2_dll.dll            │
│  0x00007FFB_80000000   kernel32.dll, ntdll.dll, ...  │
│  0x000000A0_00000000   heap                          │
│  0x000000FF_FFFFFFFF   stack (grows down)            │
│                                                     │
└─────────────────────────────────────────────────────┘
```

This means:
- A DLL can read/write the exe's memory (and vice versa)
- A pointer obtained in the exe is valid inside the DLL
- `malloc` in the DLL and `free` in the exe can cause issues only because of **different
  allocator instances**, not because of separate address spaces

### What IS Separate: Processes

**Processes** have separate address spaces. That is step05's territory (multi-process via
pipes). In step05, service1 and service2 each have their own address space — they cannot
share pointers, which is why they use JSON serialization to communicate.

### Summary Table

| Boundary | Same address space? | Can share pointers? | Communication |
|----------|-------------------|--------------------|-|
| Function call (step02) | Yes | Yes | Direct |
| DLL in same process (step03/04) | Yes | Yes* | Function call via FFI |
| Separate process (step05) | **No** | **No** | Pipes, HTTP, NATS... |

\* *Careful with allocators: if the DLL and the exe use different allocator instances, you
must free memory on the same side that allocated it.*

## Transition to Step 04

Step 04 changes the game: instead of letting Cargo statically link the rlib, the app will
**load the DLLs at runtime** using `libloading`. This means:

- The exe no longer contains the component code
- The DLLs become mandatory — if missing, the app crashes
- The app discovers components via FFI symbols (`_plugin_create`, `_plugin_destroy`)
- You could swap a DLL without recompiling the exe
