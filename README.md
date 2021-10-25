# wolrs
A wake-on-lan HTTP server implemented in rust.

## Getting started
```bash
# Start the server
$ cargo run --bin server
```

```bash
# Broadcast wake-on-lan packet
$ curl -X POST http://localhost:8000/wol?addr=$TARGET_NAC_ADDR
```
