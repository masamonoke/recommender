// @generated automatically by Diesel CLI.

diesel::table! {
    clusters (id) {
        id -> Int4,
        cluster_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    evidence_log (id) {
        id -> Int4,
        created -> Timestamp,
        content_id -> Varchar,
        event -> Varchar,
        session_id -> Varchar,
        user_id -> Int4,
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
        user_id -> Int4,
        movie_id -> Varchar,
        rating -> Numeric,
        rating_timestamp -> Timestamp,
        rating_type -> Varchar,
    }
}

diesel::table! {
    seeded_recs (id) {
        id -> Int4,
        created -> Timestamp,
        source -> Varchar,
        target -> Varchar,
        support -> Numeric,
        confidence -> Numeric,
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

diesel::joinable!(evidence_log -> movies (content_id));
diesel::joinable!(evidence_log -> users (user_id));
diesel::joinable!(movie_genre -> genre (genre_id));
diesel::joinable!(movie_genre -> movies (movie_id));

diesel::allow_tables_to_appear_in_same_query!(
    clusters,
    evidence_log,
    genre,
    movie_genre,
    movies,
    ratings,
    seeded_recs,
    users,
);
