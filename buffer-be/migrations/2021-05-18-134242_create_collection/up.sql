CREATE TABLE collections (
     id VARCHAR CONSTRAINT collections_pk PRIMARY KEY,
     user_id VARCHAR NOT NULL CONSTRAINT collections_users_id_fk REFERENCES users ON UPDATE CASCADE ON DELETE CASCADE ,
     name VARCHAR NOT NULL,
     description varchar NOT NULL,
     created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

CREATE TABLE collection_videos (
   collection_id VARCHAR NOT NULL CONSTRAINT collection_videos_collections_id_fk REFERENCES collections ON UPDATE CASCADE ON DELETE CASCADE,
   video_id VARCHAR NOT NULL CONSTRAINT collection_videos_videos_id_fk REFERENCES videos ON UPDATE CASCADE ON DELETE CASCADE,
   CONSTRAINT collection_videos_pk PRIMARY KEY (collection_id, video_id)
);
