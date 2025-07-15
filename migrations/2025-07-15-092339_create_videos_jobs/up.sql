-- Your SQL goes here
CREATE TABLE videos (
    id UUID PRIMARY KEY,
    filename TEXT NOT NULL,
    original_path TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE transcode_jobs (
    id UUID PRIMARY KEY,
    video_id UUID NOT NULL REFERENCES videos(id),
    status TEXT NOT NULL,
    output_path TEXT,
    created_at TIMESTAMP NOT NULL
);
