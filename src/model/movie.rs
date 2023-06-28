use serde::Serialize;

use crate::schema::{movies, movie_genre, genre};

#[derive(Debug, Queryable, Clone, Associations, Serialize, Identifiable, Selectable)]
#[diesel(primary_key(movie_id))]
#[diesel(table_name = movies)]
#[diesel(belongs_to(MovieGenre, foreign_key = movie_id))]
pub struct Movie {
    pub movie_id: String,
    title: String,
    year: Option<i32>,
}

#[derive(Debug, Queryable, Clone, Identifiable, Associations, Serialize, Selectable)]
#[diesel(table_name = genre)]
#[diesel(belongs_to(MovieGenre, foreign_key = id))]
pub struct Genre {
    pub id: i32,
    name: String
}

#[derive(Debug, Queryable, Identifiable, Clone)]
#[diesel(table_name = movie_genre)]
pub struct MovieGenre {
    pub id: i32,
    pub movie_id: String,
    pub genre_id: i32,
}

#[derive(Debug, Serialize)]
pub struct MovieWithGeneres {
    pub movie: Movie,
    pub genres: Vec<Genre>
} 