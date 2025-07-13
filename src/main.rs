mod config;
mod db;

use axum::{Router, routing::get};
use tracing_subscriber;
use db::connect;
use config::load_env;

#[tokio::main]
async fn main() {
    load_env();
    tracing_subscriber::fmt::init();

    let pool = connect().await;

    let app = Router::new()
        .route("/", get(|| async { "Rust Transcoder Ready" }));

    println!("🚀 Server running at http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
