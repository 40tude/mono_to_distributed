# Step 07 — Message Broker (NATS)

## Prerequisites: Install NATS Server

Unlike step06 (where your Rust code *was* the server), a message broker is a **separate
program** that runs alongside your services. You need to install it first.

### Windows (winget)

```powershell
winget install NATS.NATSServer
```

### Windows (scoop)

```powershell
scoop bucket add extras
scoop install nats-server
```

### Alternative: download binary

Download from <https://nats.io/download/> — it is a single executable, no installer needed.
Put `nats-server.exe` somewhere in your PATH.

### Verify installation

```powershell
nats-server --version
```

## Build & Run

```powershell
cargo build                    # build all workspace members
cargo test                     # unit tests (no NATS server needed)
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

## What is a Message Broker?

A message broker is a middleman that **routes messages** between producers and consumers.

In step06 (HTTP), the orchestrator needed to know the **exact address** of each service:

```
orchestrator ──POST http://127.0.0.1:3001/process──> service1
orchestrator ──POST http://127.0.0.1:3002/transform──> service2
```

With a broker, the publisher sends to a **subject** (a named channel). It has no idea who
is listening:

```
publisher ──"service.process"──> NATS broker ──> service1
publisher ──"service.transform"──> NATS broker ──> service2
```

### Why does this matter?

| Problem | HTTP (step06) | Broker (step07) |
|---------|---------------|-----------------|
| Adding a 2nd instance of service1 | Change orchestrator code or add a load balancer | Just start another service1 — NATS auto-distributes |
| Service1 restarts | Orchestrator gets a connection error | Publisher retries; NATS delivers when service1 is back |
| Service moves to another machine | Update the URL in orchestrator | Nothing changes — it subscribes to the same subject |

### Key Concepts

- **Subject**: a string like `"service.process"` — the "address" of a message. Not a URL,
  not an IP. Just a name.
- **Publish**: send a message to a subject.
- **Subscribe**: listen for messages on a subject.
- **Request/Reply**: publish a message and wait for exactly one reply. NATS handles this by
  creating a temporary "inbox" subject behind the scenes.

## What Changed from Step 06

### What was removed

- **No HTTP server** — services no longer run Axum. No ports, no routes, no
  `Router::new()`. The NATS client library handles all networking.
- **No URLs** — the publisher does not know `http://127.0.0.1:3001`. It only knows the
  subject name `"service.process"`.
- **No reqwest** — the publisher uses `client.request()` from async-nats instead of HTTP
  POST.

### What was added

- **NATS client** (`async-nats` crate) — replaces both Axum (server) and reqwest (client).
  Both the publisher and the services use the same `async_nats::connect()`.
- **Subjects** defined in `common/src/lib.rs` — `SUBJECT_PROCESS` and `SUBJECT_TRANSFORM`.
- **Timeout** on requests — since the broker is async, the publisher wraps each request in
  `tokio::time::timeout()` to avoid waiting forever if a service is down.

### What stayed the same

- **Business logic** — identical. Input 42, multiply by 2, format as "Value-0084".
- **Data types** — `ProcessRequest`, `ProcessResponse`, `TransformRequest`,
  `TransformResponse` are unchanged from step06.
- **Serialization** — still JSON via serde. The bytes go through NATS instead of HTTP
  bodies.
- **Unit tests** — same tests, no NATS server needed (they test pure business logic).

## Dependency Graph

```
            common              (shared types + subject constants)
           /  |  \
   service1  service2  publisher
       \       |       /
        \      |      /
       NATS broker (external)
```

## Architecture Comparison

```
Step 06 (HTTP — point to point):

  orchestrator ──HTTP POST──> service1 (Axum on :3001)
  orchestrator ──HTTP POST──> service2 (Axum on :3002)

Step 07 (NATS — broker mediated):

  publisher ──publish──> NATS server <──subscribe── service1
  publisher ──publish──> NATS server <──subscribe── service2
```

## How Request/Reply Works Internally

When you call `client.request("service.process", payload)`:

1. The NATS client creates a **temporary inbox** subject (e.g., `_INBOX.abc123`)
2. It **subscribes** to that inbox
3. It **publishes** your payload to `"service.process"` with a reply-to header pointing to
   the inbox
4. Service1 receives the message, processes it, and **publishes** the response to the
   reply-to subject
5. The publisher's inbox subscription receives the response
6. `client.request()` returns it to you

All of this is hidden behind one line of code. You never see the inbox.

## Going Further

This example uses **request/reply** (one message, one answer) to match the HTTP pattern
from step06. But NATS also supports:

- **Pub/Sub** — one message, multiple consumers (fan-out)
- **Queue Groups** — multiple instances of the same service, NATS distributes load
  automatically
- **JetStream** — persistent messages that survive broker restarts (like Kafka)

These patterns have no equivalent in plain HTTP.
