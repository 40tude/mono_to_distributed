```powershell
cargo build       # build all workspace members including service1 & service2
cargo run -p app  # run app (requires service1 & service2 built first)
cargo test
cargo test -p service2
```
