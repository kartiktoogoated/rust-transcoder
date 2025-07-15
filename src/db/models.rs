use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::db::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = videos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Video {
    #[diesel(sql_type = DieselUuid)]
    pub id: Uuid,
    pub filename: String,
    pub original_path: String,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = transcode_jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TranscodeJob {
    #[diesel(sql_type = DieselUuid)]
    pub id: Uuid,
    #[diesel(sql_type = DieselUuid)]
    pub video_id: Uuid,
    pub status: String,
    pub output_path: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}
