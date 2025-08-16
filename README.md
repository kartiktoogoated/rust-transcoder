# rust-transcoder
<img width="463" height="293" alt="Screenshot 2025-08-16 at 3 22 46 PM" src="https://github.com/user-attachments/assets/a46cfacd-eacf-4586-b320-0b4f13f56fd4" />

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
