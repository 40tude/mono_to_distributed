# Step 02.5 — Trait-Based Interface (Shared Contracts)

## Build & Run

```powershell
cargo build                    # build all workspace members
cargo run
cargo test
```

## What Changed from Step 02

In step 02, `app` calls `Component1` and `Component2` methods **directly**. The app knows
every concrete type and every method signature. If you swap an implementation, you must
change the app code too.

Step 02.5 introduces a **`traits` crate** that sits between the app and the component
libraries. It defines two trait contracts:

- `Processor` — `process(i32) -> ProcessResult` + `validate(&ProcessResult) -> bool`
- `Transformer` — `transform(i32) -> TransformResult` + `analyze(&TransformResult) -> String`

The data types (`ProcessResult`, `TransformResult`) also move into `traits`, since they are
part of the shared contract.

## Why This Step Matters

1. **Decoupling** — `app` can now call components through `&dyn Processor` / `&dyn Transformer`.
   The `run_pipeline()` function in main.rs depends only on `traits`, not on any concrete
   component. You could swap Component1 for a completely different implementation without
   touching `run_pipeline`.

2. **Dependency inversion** — Components depend on the trait crate (upward), not the other
   way around. The app also depends on the trait crate. No component depends on another
   component.

3. **Prepares for plugins** — Step 03/04 will turn components into DLLs. Having a clean
   trait boundary now makes that transition natural: the trait crate becomes the plugin
   interface, and each DLL just provides a different `impl`.

## Dependency Graph

```
              traits          (defines Processor, Transformer, data types)
             /      \
   component1_lib  component2_lib   (each impl one trait)
             \      /
               app                  (uses both through trait references)
```

## Key Diff Summary

| File | Change |
|------|--------|
| `traits/src/lib.rs` | **NEW** — `Processor` trait, `Transformer` trait, shared data types |
| `component1_lib/src/lib.rs` | `Component1Data` removed, now uses `ProcessResult` from traits. Methods moved into `impl Processor for Component1` |
| `component2_lib/src/lib.rs` | `Component2Data` removed, now uses `TransformResult` from traits. Methods moved into `impl Transformer for Component2` |
| `app/src/main.rs` | Imports traits. Adds `run_pipeline(&dyn Processor, &dyn Transformer, i32)` to demonstrate polymorphism |
| `Cargo.toml` (workspace) | Added `traits` to workspace members |

## What to Notice

- The **business logic is identical** to step 02. Same input (42), same output ("Value-0084").
- `run_pipeline()` uses **dynamic dispatch** (`&dyn Trait`). This is the same mechanism
  that step 04 will use when loading plugins from DLLs at runtime.
- The `traits` crate has **zero dependencies** — it is a pure contract definition.
