# wolrs
A wake-on-lan HTTP server implemented in rust.

## Getting started

- Start the server
```bash
$ cargo run --bin server
```

- Broadcast wake-on-lan packet
```bash
$ curl -X POST http://localhost:8000/wol?addr=$TARGET_MAC_ADDR
```
