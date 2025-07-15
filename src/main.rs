mod config;
mod db;
mod api;
mod tasks;

use api::{upload::router as upload_router, job::router as job_router};
use axum::{Router, routing::get};
use axum::extract::DefaultBodyLimit;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber;
use db::connect;
use std::fs;
use config::load_env;


#[tokio::main]
async fn main() {
    load_env();
    tracing_subscriber::fmt::init();

    fs::create_dir_all("outputs").unwrap();

    let pool = connect();

    let app = Router::new()
    .route("/", get(|| async { "Rust Transcoder Ready" }))
    .nest("/api", upload_router(pool.clone()).merge(job_router(pool.clone())))
    .layer(DefaultBodyLimit::max(100 * 1024 * 1024));

    println!("🚀 Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
