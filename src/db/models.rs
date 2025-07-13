use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: Uuid,
    pub filename: String,
    pub original_path: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TranscodeJob {
    pub id: Uuid,
    pub video_id: Uuid,
    pub status: String,
    pub output_path: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
