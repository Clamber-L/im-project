# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build all
cargo build

# Build a specific service
cargo build -p chat-server

# Run a service
cargo run -p server-auth

# Run tests
cargo test
cargo test -p lib-core          # single crate

# Check without building
cargo check

# Lint
cargo clippy

# After modifying .proto files, rebuild (tonic-build runs in build.rs)
cargo build -p lib-rpc
```

## Architecture

This is a Rust Cargo workspace implementing a microservices IM (instant messaging) backend.

### Workspace layout

```
creates/          # shared libraries
  lib-core/       # DB clients, JWT, Redis, middleware, Nacos, logger
  lib-entity/     # SeaORM (MySQL) and MongoDB entity definitions
  lib-rpc/        # tonic gRPC client pools + generated protobuf bindings
  lib-utils/      # config loading, date, password, result helpers

service/          # runnable microservices
  chat-server     # WebSocket endpoint + gRPC MessageService server
  net-server      # multi-protocol listener: TCP/8080, WS/8081, MQTT/1883
  server-auth     # HTTP login/auth (Axum, MySQL)
  server-message  # message push service
  server-user     # HTTP user API (Axum, MySQL); receives gRPC from chat-server
  server-wechat   # WeChat Pay / Mini Program integration (HTTP/8888)
  pingora-project # Pingora reverse proxy / load balancer
  rust-cli        # CLI tooling (clap + dialoguer)

protos/           # .proto source files (message/, user/)
```

### Service interaction flow

1. Clients connect to **chat-server** via WebSocket (`/connect/:token`).
2. **chat-server** calls **server-user** over gRPC to persist messages and trigger push notifications.
3. **server-user** forwards notifications to **server-message** via gRPC.
4. **net-server** is an alternative multi-protocol gateway that proxies to **server-user** via a gRPC connection pool (bb8).
5. **server-auth** is stateless HTTP; it issues JWTs verified by `lib-core::verification_token`.

### Config

Each service loads a YAML file via its config struct in `lib-utils/src/config/` (e.g. `ServerChatConfig::try_load()`). Config files are co-located with the binary or pointed to by an env var. See `server-wechat.yml` for a reference example.

### Key dependencies

| Concern | Crate |
|---|---|
| HTTP | axum 0.8 |
| gRPC | tonic 0.12 + prost |
| MySQL ORM | sea-orm (sqlx-mysql) |
| MongoDB | mongodb 3 |
| Redis | redis 0.25 (tokio) |
| gRPC conn pool | bb8 |
| Service discovery | nacos-sdk |
| Reverse proxy | pingora 0.4 |
| ID generation | sonyflake |
| Auth | jwt + argon2 |
