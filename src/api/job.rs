use axum::{
    extract::{Path, State},
    response::{IntoResponse},
    http::StatusCode,
    Json, Router,
    routing::get,
};
use uuid::Uuid;
use diesel::prelude::*;
use crate::db::{DbPool};
use crate::db::models::TranscodeJob;
use crate::db::schema::transcode_jobs;

#[derive(Debug, serde::Serialize)]
pub struct JobStatusResponse {
    pub job_id: Uuid,
    pub status: String,
    pub output_path: Option<String>,
}

pub fn router(pool: DbPool) -> Router {
    Router::new().route("/jobs/{id}", get(get_job)).with_state(pool)
}

async fn get_job(
    State(pool): State<DbPool>,
    Path(job_id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = pool.clone();

    let job_result = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().unwrap();
        transcode_jobs::table
            .find(job_id)
            .first::<TranscodeJob>(&mut conn)
    }).await.unwrap();

    match job_result {
        Ok(job) => {
            Json(JobStatusResponse {
                job_id: job.id,
                status: job.status,
                output_path: job.output_path,
            }).into_response()
        },
        Err(_) => (StatusCode::NOT_FOUND, "Job not found").into_response(),
    }
}