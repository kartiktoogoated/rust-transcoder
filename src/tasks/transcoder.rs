use std::path::{PathBuf};
use uuid::Uuid;
use std::process::Stdio;
use diesel::prelude::*;
use tokio::process::Command;
use tracing::{info, error};

use crate::db::{DbPool};
use crate::db::schema::transcode_jobs;

pub async fn start_transcoding(
    pool: DbPool,
    video_path: String,
    job_id: Uuid,
) {
    let output_path = format!("outputs/{}.mp4", job_id);
    let _output_file = PathBuf::from(&output_path);

    // Spawn FFmpeg command
    let output = Command::new("ffmpeg")
        .args([
            "-y", // overwrite
            "-i", &video_path, // input
            "-c:v", "libx264",
            "-preset", "fast",
            "-c:a", "aac",
            &output_path,
        ])
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            info!("Transcoding completed for job {}", job_id);
            let pool_clone = pool.clone();
            let update_result = tokio::task::spawn_blocking(move || {
                let mut conn = pool_clone.get().unwrap();
                diesel::update(transcode_jobs::table.find(job_id))
                    .set((
                        transcode_jobs::status.eq("completed"),
                        transcode_jobs::output_path.eq(Some(output_path)),
                    ))
                    .execute(&mut conn)
            }).await.unwrap();

            if let Err(e) = update_result {
                error!("Failed to update job status: {}", e)
            }
        }
        Ok(output) => {
            error!(
                "Transcoding failed for job {}: {}",
                job_id,
                String::from_utf8_lossy(&output.stderr)
            );
            let pool_clone = pool.clone();
            let _ = tokio::task::spawn_blocking(move || {
                let mut conn = pool_clone.get().unwrap();
                diesel::update(transcode_jobs::table.find(job_id))
                    .set(transcode_jobs::status.eq("failed"))
                    .execute(&mut conn)
            }).await;
        }
        Err(e) => {
            error!("Failed to spawn ffmpeg for job {}: {}", job_id, e);
            let pool_clone = pool.clone();
            let _ = tokio::task::spawn_blocking(move || {
                let mut conn = pool_clone.get().unwrap();
                diesel::update(transcode_jobs::table.find(job_id))
                    .set(transcode_jobs::status.eq("failed"))
                    .execute(&mut conn)
            }).await;
        }
    }
}