table! {
    creators (id) {
        id -> Varchar,
        user_id -> Int4,
    }
}

table! {
    followers (creator_id, viewer_id) {
        creator_id -> Varchar,
        viewer_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        display_name -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    videos (id) {
        id -> Varchar,
        uploader -> Int4,
        title -> Varchar,
        description -> Text,
        video_path -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(creators -> users (user_id));
joinable!(followers -> creators (creator_id));
joinable!(followers -> users (viewer_id));
joinable!(videos -> users (uploader));

allow_tables_to_appear_in_same_query!(
    creators,
    followers,
    users,
    videos,
);
