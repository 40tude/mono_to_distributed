Prerequisites: Install NATS Server

```powershell
winget install NATS.NATSServer
```

Verify installation

```powershell
nats-server --version
```

```powershell
cargo build
cargo test
```

To run the full pipeline, open **4 terminals**:

```powershell
# Terminal 1 — Start the NATS broker
nats-server

# Terminal 2
cargo run -p service1          # subscribes to "service.process"

# Terminal 3
cargo run -p service2          # subscribes to "service.transform"

# Terminal 4
cargo run -p publisher         # sends requests, prints results
```