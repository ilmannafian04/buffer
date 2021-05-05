CREATE TABLE ratings (
	video_id VARCHAR NOT NULL,
	user_id VARCHAR NOT NULL,
    is_dislike BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT ratings_pk PRIMARY KEY (video_id, user_id)
);
