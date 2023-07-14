use serde::Serialize;

use crate::schema::{movies, movie_genre, genre};

#[derive(Debug, Queryable, Clone, Serialize, Identifiable, Selectable, PartialEq)]
#[diesel(primary_key(movie_id))]
#[diesel(table_name = movies)]
pub struct Movie {
    pub movie_id: String,
    pub title: String,
    pub year: Option<i32>,
}

#[derive(Debug, Queryable, Clone, Identifiable, Associations, Serialize, Selectable)]
#[diesel(table_name = genre)]
#[diesel(belongs_to(MovieGenre, foreign_key = id))]
pub struct Genre {
    pub id: i32,
    pub name: String
}

#[derive(Debug, Queryable, Identifiable, Clone, Associations, Selectable)]
#[diesel(table_name = movie_genre)]
#[diesel(belongs_to(Movie, foreign_key = movie_id))]
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
