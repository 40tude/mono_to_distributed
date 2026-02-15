# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Educational project: 9 progressive steps (step00–step08) evolving from monolith to distributed Rust architecture. Each step is a **self-contained Cargo workspace**. Associated blog: https://www.40tude.fr/docs/06_programmation/rust/026_monolith_to_distributed/monolith_to_distirbuted.html

## Build Commands

Each step is independent. `cd` into a step folder first.

```bash
cargo build                    # build all workspace members
cargo build -p <crate>         # build single crate
cargo run                      # run default binary (monolithic steps)
cargo run -p <crate>           # run specific crate (distributed steps)
cargo test                     # run all tests
cargo test -p <crate>          # test single crate
cargo clippy --workspace       # lint all crates
```

**Windows `.cargo/` warning**: each step may contain a `.cargo/` folder with OneDrive-specific `target-dir` and CPU flags. Rename before building on other setups: `mv .cargo .cargo.bak`

## Rust Edition

All crates use **edition 2024** with **resolver 3**.

## Architecture Progression

All steps share identical business logic: `i32 → process (×2) → transform (format string)`.

| Step | Architecture | Communication | Key Concept |
|------|-------------|---------------|-------------|
| 00 | Single file | Direct calls | Baseline monolith |
| 01 | Multi-file | Direct calls | Module organization |
| 02 | Workspace (3 crates) | Direct calls | Modular monolith, library crates |
| 03 | Workspace + traits crate | `&dyn Trait` | Dependency inversion, trait objects |
| 04 | Same as 03 + `cdylib` | Static linking (rlib used) | DLL artifact generation (not loaded) |
| 05 | Runtime DLL loading | FFI via `libloading` | C-compatible symbols, dynamic plugins |
| 06 | Multi-process | JSON over stdin/stdout pipes | Process spawning, serde IPC |
| 07 | HTTP services | REST via Axum | Services on ports 3001/3002 |
| 08 | Message broker | NATS pub/sub + request/reply | Decoupled services, scalability |

## Running Distributed Steps (06–08)

- **Step 06**: Build service1/service2 first, then `cargo run -p orchestrator` (spawns children)
- **Step 07**: Start service1 & service2 in separate terminals, then run orchestrator
- **Step 08**: Requires NATS server (`winget install NATS.NATSServer`). Start: NATS → service1 → service2 → app (4 terminals)

Each step has a `QUICK_START.md` with exact run instructions.

## Ignored Directories

- `temp/` — experimental step variants (not part of the project)
- `docs/` — blog article drafts (`docs/stepXX.md`), one per step

## Key Dependencies

- `serde`/`serde_json` — serialization (steps 02+)
- `tokio` — async runtime (steps 06+)
- `axum` — HTTP framework (step 07)
- `libloading` — runtime DLL loading (step 05)
- `async-nats` — NATS client (step 08)

## Step-Specific Constraints

- **Step 05**: `plugin_interface` crate defines C-compatible FFI symbols; components are `cdylib` only
- **Step 06**: Orchestrator hardcodes exe paths from `.cargo/config.toml` `target-dir` — update `HEADER` const in `orchestrator/src/main.rs` if paths change
- **Step 07**: Service endpoints: `POST /process` (port 3001), `POST /transform` (port 3002), `GET /health`
- **Step 08**: NATS subjects defined in `common/src/lib.rs` (`SUBJECT_PROCESS`, `SUBJECT_TRANSFORM`). `app` crate is the orchestrator (replaces former `publisher`)
