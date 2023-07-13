use crate::{diesel, schema::{movies, genre}, model::movie::{Movie, Genre, MovieWithGeneres}};
use diesel::prelude::*;
use crate::{schema::movie_genre, model::movie::MovieGenre};

// TODO: optimize query
pub fn get_all_movies(connection: &mut PgConnection) -> Vec<MovieWithGeneres> {
    let movies_genres = movie_genre::table
        .order(movie_genre::columns::id.asc())
        .load::<MovieGenre>(connection)
        .unwrap();

    let all_movies = movies::table
        .select(Movie::as_select())
        .load(connection)
        .unwrap();

    let genres = genre::table.select(Genre::as_select()).load(connection).unwrap();

    let mut movies: Vec<MovieWithGeneres> = vec![];
    for movie in all_movies {
        let tmp_movie_generes:Vec<&MovieGenre> = movies_genres
            .iter()
            .filter(|v| movie.movie_id.eq(&v.movie_id))
            .collect::<Vec<&MovieGenre>>();
        let mut movie_with_genre = MovieWithGeneres { movie: movie.clone(), genres: vec![]};
        for tmp_movie_genre in tmp_movie_generes {
            let mut genres = genres.iter().filter(|genre| genre.id == tmp_movie_genre.genre_id).cloned().collect::<Vec<Genre>>();
            movie_with_genre.genres.append(&mut genres);
        }
        movies.push(movie_with_genre);
    }

    movies
}
