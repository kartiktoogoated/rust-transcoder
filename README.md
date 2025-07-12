# rust-transcoder

rust_transcoder/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point
│   ├── api/                 # Axum handlers
│   │   ├── mod.rs
│   │   └── video.rs
│   ├── db/                  # Database logic
│   │   ├── mod.rs
│   │   └── models.rs        # SQLx structs + queries
│   ├── ffmpeg/              # Transcoder logic
│   │   └── mod.rs
│   ├── tasks/               # Background task runner
│   └── config.rs            # Env + config loading

| Purpose             | Tool                                |
| ------------------- | ----------------------------------- |
| HTTP Server         | `Axum`                              |
| Async Runtime       | `Tokio`                             |
| Database ORM        | `SQLx` with PostgreSQL              |
| File upload parsing | `multipart`                         |
| UUIDs               | `uuid` crate                        |
| Logging             | `tracing`                           |
| Background tasks    | `tokio::spawn`, or real queue later |
| Video processing    | FFmpeg via `Command`                |
| Optional            | S3 / Cloud upload later             |
