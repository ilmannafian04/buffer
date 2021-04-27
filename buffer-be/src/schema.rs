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

joinable!(videos -> users (uploader));

allow_tables_to_appear_in_same_query!(
    users,
    videos,
);
