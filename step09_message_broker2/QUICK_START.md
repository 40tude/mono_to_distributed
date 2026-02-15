## NATS Quick Guide

* https://docs.nats.io/

### Prerequisites: Install NATS Server

* Unlike Step07 (where our Rust code *was* the server), a message broker is a **separate program** that runs alongside our services. We need to install it first.
* https://docs.nats.io/running-a-nats-service/introduction/installation
* https://www.bowmanjd.com/chocolatey-scoop-winget/


### Install using scoop (Windows)

```powershell
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
iwr -useb get.scoop.sh | iex
scoop bucket add extras
scoop install main/nats-server
# Verify installation
nats-server --version
```

**Note:** In order to run `nats-server --version` in a brand new VSCode terminal I had to quit and reload VSCode. I tried many things before reload without success. It's a kind of magic 🎹.


### Starting NATS

**Option 1: Simple foreground start**
```powershell
nats-server
```
This starts NATS with default config (port 4222).
Press `Ctrl+C` to stop.
The server will handle `CTRL+C` gracefully


**Option 2: Start with custom config**
```powershell
nats-server -c path\to\config.conf
```

**Option 3: Background process (recommended for dev)**
```powershell
# Start in background
Start-Process nats-server -WindowStyle Hidden

# Or with config
Start-Process nats-server -ArgumentList "-c", "path\to\config.conf" -WindowStyle Hidden
```

### Stopping NATS

```powershell
# Find NATS process
Get-Process nats-server

# Stop it
Stop-Process -Name nats-server

# Or force kill if needed
Stop-Process -Name nats-server -Force
```

### Quick Health Check

```powershell
# Check if NATS is running
Get-Process nats-server -ErrorAction SilentlyContinue

# Test connection (requires curl or similar)
curl http://localhost:8222/varz
```

## Useful Tips

- **Default port**: 4222 (client connections)
- **Monitoring port**: 8222 (HTTP monitoring endpoint)
- **Config location**: NATS looks for `nats-server.conf` in current directory by default
- **Logs**: Displayed in console by default, configure file logging in config if needed

## For Production/Services

If you want NATS as a Windows service, you'd need to use something like NSSM:
```powershell
scoop install nssm
nssm install nats-server "$(scoop which nats-server)"
```















## Build & Run

```powershell
cargo build             # build all workspace members
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
cargo run -p app         # sends requests, prints results
```

The publisher will exit after printing the pipeline result. The services keep running,
waiting for more messages. You can run the publisher again — same services will handle it.
