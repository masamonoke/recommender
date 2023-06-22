use crate::{diesel, schema::{movies, genre}, model::movie::movie::{Movie, Genre, MovieWithGeneres}};
use diesel::prelude::*;
use actix_web::{HttpRequest, Responder, web};
use crate::{database::establish_connection, schema::movie_genre, model::movie::movie::MovieGenre};

pub async fn get(_req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    
    let movies_genres = movie_genre::table
        .order(movie_genre::columns::id.asc())
        .load::<MovieGenre>(&mut connection)
        .unwrap();

    let all_movies = movies::table.select(Movie::as_select()).load(&mut connection).unwrap();

    let genres = genre::table.select(Genre::as_select()).load(&mut connection).unwrap();

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

    web::Json(movies)
}
