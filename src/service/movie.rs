use std::collections::HashMap;

use crate::{diesel, schema::{movies, genre}, model::movie::{Movie, Genre, MovieWithGeneres}};
use diesel::prelude::*;
use crate::model::movie::MovieGenre;

pub fn get_limited_movies(connection: &mut PgConnection, limit: i64) -> Vec<MovieWithGeneres> {
    let limited_movies = movies::table
        .order(movies::columns::year.desc())
        .select(Movie::as_select())
        .limit(limit)
        .load(connection)
        .unwrap();

    get_movies_with_genres_from_movies_list(connection, limited_movies)
}

pub fn get_movie_by_id(connection: &mut PgConnection, movie_id: String) -> Option<MovieWithGeneres> {
    let movies_list = movies::table
        .select(Movie::as_select())
        .filter(movies::columns::movie_id.eq(movie_id))
        .load(connection)
        .unwrap();

    let movie: Vec<MovieWithGeneres> = get_movies_with_genres_from_movies_list(connection, movies_list);
    movie.first().cloned()
}

pub fn get_movies_with_genres_from_movies_list(connection: &mut PgConnection, movies_list: Vec<Movie>) -> Vec<MovieWithGeneres> {
    let movies_genres = MovieGenre::belonging_to(&movies_list)
    .select(MovieGenre::as_select())
    .load(connection)
    .unwrap();

    let genres_map: HashMap<i32, Genre> = genre::table
        .select(Genre::as_select())
        .load(connection)
        .unwrap()
        .iter()
        .map(|genre| (genre.id, genre.to_owned()))
        .collect();

    let genres_per_movie = movies_genres
        .grouped_by(&movies_list)
        .into_iter()
        .zip(movies_list)
        .map(|(genres, movie)| {
            let movie_genres: Vec<Genre> = genres
                .iter()
                .map(|genre| genres_map.get(&genre.genre_id).unwrap().clone())
                .collect::<Vec<Genre>>();

            MovieWithGeneres { movie, genres: movie_genres }
        })
        .collect::<Vec<MovieWithGeneres>>();

    genres_per_movie
}

