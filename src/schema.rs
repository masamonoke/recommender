// @generated automatically by Diesel CLI.

diesel::table! {
    evidence_log (id) {
        id -> Int4,
        created -> Timestamp,
        content_id -> Varchar,
        event -> Varchar,
        session_id -> Varchar,
        user_id -> Varchar,
    }
}

diesel::table! {
    genre (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    movie_genre (id) {
        id -> Int4,
        movie_id -> Varchar,
        genre_id -> Int4,
    }
}

diesel::table! {
    movies (movie_id) {
        movie_id -> Varchar,
        title -> Varchar,
        year -> Nullable<Int4>,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        user_id -> Varchar,
        movie_id -> Varchar,
        rating -> Int4,
        rating_timestamp -> Timestamp,
        rating_type -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        unique_id -> Varchar,
    }
}

diesel::joinable!(movie_genre -> genre (genre_id));
diesel::joinable!(movie_genre -> movies (movie_id));

diesel::allow_tables_to_appear_in_same_query!(
    evidence_log,
    genre,
    movie_genre,
    movies,
    ratings,
    users,
);
