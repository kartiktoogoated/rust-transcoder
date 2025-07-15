use axum::{
    Json, Router,
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::multipart::Multipart;
use serde::{Serialize, Deserialize};
use crate::db::DbPool;
use diesel::prelude::*;
use crate::db::models::{Video, TranscodeJob};
use crate::db::schema::{videos, transcode_jobs};
use std::{fs::create_dir_all, path::Path};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    job_id: Uuid,
    message: String,
}

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/upload", post(upload))
        .with_state(pool)
}

async fn upload(State(pool): State<DbPool>, mut multipart: Multipart) -> impl IntoResponse {
    let upload_dir = Path::new("uploads");
    create_dir_all(upload_dir).unwrap();
    let pool = pool.clone();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("video.mp4").to_string();
        let data = field.bytes().await.unwrap();

        let file_path = upload_dir.join(&file_name);
        tokio::fs::write(&file_path, &data).await.unwrap();

        let video_id = Uuid::new_v4();
        let job_id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        // Insert a video using Diesel
        let new_video = Video {
            id: video_id,
            filename: file_name.clone(),
            original_path: file_path.to_string_lossy().to_string(),
            created_at: now,
        };
        let video_pool = pool.clone();
        let insert_video = tokio::task::spawn_blocking(move || {
            let mut conn = video_pool.get().expect("Failed to get DB connection");
            diesel::insert_into(videos::table)
                .values(&new_video)
                .execute(&mut conn)
        }).await.unwrap();
        if let Err(e) = insert_video {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)).into_response();
        }

        // Insert transcode job using Diesel
        let new_job = TranscodeJob {
            id: job_id,
            video_id,
            status: "queued".to_string(),
            output_path: None,
            created_at: now,
        };
        let job_pool = pool.clone();
        let insert_job = tokio::task::spawn_blocking(move || {
            let mut conn = job_pool.get().expect("Failed to get DB connection");
            diesel::insert_into(transcode_jobs::table)
                .values(&new_job)
                .execute(&mut conn)
        }).await.unwrap();
        if let Err(e) = insert_job {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)).into_response();
        }

        return (
            StatusCode::OK,
            Json(UploadResponse {
                job_id,
                message: "Video uploaded successfully".into(),
            }),
        ).into_response();
    }

    (StatusCode::BAD_REQUEST, "No file found").into_response()
}
