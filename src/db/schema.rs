// @generated automatically by Diesel CLI.

diesel::table! {
    transcode_jobs (id) {
        id -> Uuid,
        video_id -> Uuid,
        status -> Text,
        output_path -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    videos (id) {
        id -> Uuid,
        filename -> Text,
        original_path -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(transcode_jobs -> videos (video_id));

diesel::allow_tables_to_appear_in_same_query!(
    transcode_jobs,
    videos,
);
