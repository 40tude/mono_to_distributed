
```powershell
cargo build             # build all workspace members
cargo test
cargo test -p service2
```

To run the full pipeline, open **3 terminals** in VSCode

```powershell

# Terminal 1
cargo run -p service1   # starts on http://127.0.0.1:3001

# Terminal 2
cargo run -p service2   # starts on http://127.0.0.1:3002

# Terminal 3
cargo run -p app        # sends HTTP requests to service1 & service2

```

* Check the content of the terminal where `service1` and `service2` are running
* Run the `app` many times
* Close the terminal of the `app`
* CTRL+C to stop the `service1`, close the terminal
* CTRL+C to stop the `service2`, close the terminal