table! {
    comments (id) {
        id -> Varchar,
        video_id -> Varchar,
        user_id -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    creators (user_id) {
        user_id -> Varchar,
    }
}

table! {
    followers (creator_id, viewer_id) {
        creator_id -> Varchar,
        viewer_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    ratings (video_id, user_id) {
        video_id -> Varchar,
        user_id -> Varchar,
        is_dislike -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
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
        uploader -> Varchar,
        title -> Varchar,
        description -> Text,
        video_path -> Varchar,
        thumbnail_path -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(comments -> users (user_id));
joinable!(comments -> videos (video_id));
joinable!(creators -> users (user_id));
joinable!(followers -> creators (creator_id));
joinable!(followers -> users (viewer_id));
joinable!(videos -> users (uploader));

allow_tables_to_appear_in_same_query!(
    comments,
    creators,
    followers,
    ratings,
    users,
    videos,
);
