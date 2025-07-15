use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_typed_multipart::{TypedMultipart, TryFromMultipart, FieldData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{fs::create_dir_all, path::Path};
use crate::db::{
    DbPool,
    models::{Video, TranscodeJob},
    schema::{videos, transcode_jobs},
};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub job_id: Uuid,
    pub message: String,
}

#[derive(Debug, TryFromMultipart)]
pub struct UploadForm {
    pub file: FieldData<axum::body::Bytes>,
}

// Export the router
use axum::routing::post;
use axum::Router;

pub fn router(pool: DbPool) -> Router {
    Router::new()
        .route("/upload", post(upload))
        .with_state(pool)
}

pub async fn upload(
    State(pool): State<DbPool>,
    TypedMultipart(form): TypedMultipart<UploadForm>,
) -> impl IntoResponse {
    let file_name = form.file.metadata.file_name.clone().unwrap_or("video.mp4".to_string());
    let content = &form.file.contents;
    let upload_dir = Path::new("uploads");
    create_dir_all(upload_dir).unwrap();
    let file_path = upload_dir.join(&file_name);
    tokio::fs::write(&file_path, content).await.unwrap();

    let video_id = Uuid::new_v4();
    let job_id = Uuid::new_v4();
    let now = chrono::Utc::now().naive_utc();

    let new_video = Video {
        id: video_id,
        filename: file_name.clone(),
        original_path: file_path.to_string_lossy().to_string(),
        created_at: now,
    };

    let insert_video = tokio::task::spawn_blocking({
        let pool = pool.clone();
        move || {
            let mut conn = pool.get().unwrap();
            diesel::insert_into(videos::table)
                .values(&new_video)
                .execute(&mut conn)
        }
    })
    .await
    .unwrap();

    if let Err(e) = insert_video {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB error: {}", e),
        )
            .into_response();
    }

    let new_job = TranscodeJob {
        id: job_id,
        video_id,
        status: "queued".to_string(),
        output_path: None,
        created_at: now,
    };

    let insert_job = tokio::task::spawn_blocking({
        let pool = pool.clone();
        move || {
            let mut conn = pool.get().unwrap();
            diesel::insert_into(transcode_jobs::table)
                .values(&new_job)
                .execute(&mut conn)
        }
    })
    .await
    .unwrap();

    if let Err(e) = insert_job {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB error: {}", e),
        )
            .into_response();
    }

    // Start transcoding task
    let video_path_clone = file_path.to_string_lossy().to_string();
    let ffmpeg_pool = pool.clone();
    tokio::spawn(async move {
        crate::tasks::transcoder::start_transcoding(
            ffmpeg_pool,
            video_path_clone,
            job_id,
        )
        .await;
    });

    (
        StatusCode::OK,
        Json(UploadResponse {
            job_id,
            message: "✅ Video uploaded successfully".into(),
        }),
    )
        .into_response()
}
