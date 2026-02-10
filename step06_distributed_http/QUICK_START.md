```powershell
cargo build                    # build all workspace members

# In 3 different terminals
cargo run -p service1          # starts on http://127.0.0.1:3001
cargo run -p service2          # starts on http://127.0.0.1:3002
cargo run -p orchestrator      # sends HTTP requests to service1 & service2

```
