## Prerequisites: Install NATS Server

* Unlike Step07 (where our Rust code *was* the server), a message broker is a **separate program** that runs alongside our services. We need to install it first.
* https://docs.nats.io/running-a-nats-service/introduction/installation
* https://www.bowmanjd.com/chocolatey-scoop-winget/


### Windows (scoop)

```powershell
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
iwr -useb get.scoop.sh | iex
scoop bucket add extras
scoop install main/nats-server
```

### Verify installation

```powershell
nats-server --version
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
cargo run -p publisher         # sends requests, prints results
```

The publisher will exit after printing the pipeline result. The services keep running,
waiting for more messages. You can run the publisher again — same services will handle it.
