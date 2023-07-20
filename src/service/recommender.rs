use diesel::{ExpressionMethods, RunQueryDsl, PgConnection};
use diesel::prelude::*;

use crate::model::movie::MovieWithGeneres;
use crate::model::recs::SeededRec;
use crate::model::{movie::Movie, evidence::Log};
// TODO: remove dsl import
use crate::schema::evidence_log::{dsl::*, self};
use crate::schema::{movies, seeded_recs};

use super::movie::{get_movie_by_id, get_movies_with_genres_from_movies_list};

use itertools::Itertools;

struct MovieCounted {
    movie: Movie,
    count: usize
}

impl Eq for MovieCounted {}

impl PartialEq for MovieCounted {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl PartialOrd for MovieCounted {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other.count.partial_cmp(&self.count).unwrap() == std::cmp::Ordering::Equal {
            let y1 = other.movie.year.unwrap_or_default();
            let y2 = self.movie.year.unwrap_or_default();
            y1.partial_cmp(&y2)
        } else {
            other.count.partial_cmp(&self.count)
        }
    }
}

impl std::cmp::Ord for MovieCounted {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.count.cmp(&self.count)
    }
}

//heavy request should be cached for all sessions and execute only several times per day, maybe per week
pub fn chart(connection: &mut PgConnection) -> Vec<Movie> {
    let all_movies = movies::table
        .select(Movie::as_select())
        .load(connection)
        .unwrap();

    let all_movies = get_movies_with_genres_from_movies_list(connection, all_movies);

    let logs = evidence_log::table
        .select(Log::as_select())
        .filter(event.eq("buy"))
        .load(connection)
        .unwrap();

    let mut counted_movies = vec![];
    for movie in all_movies {
        let movie_buys_count = logs.iter().filter(|l| l.content_id == movie.movie.movie_id).count();
        let value = MovieCounted { movie: movie.movie, count: movie_buys_count };
        counted_movies.push(value);
    }
    counted_movies.sort();

    let top_movies: Vec<Movie> = counted_movies.iter().map(|v| v.movie.clone()).take(10).collect();
    
    top_movies
}

pub fn get_associated_with_objects(connection: &mut PgConnection, movie_id: String) -> Vec<MovieWithGeneres>{
    seeded_recs::table
        .select(SeededRec::as_select())
        .filter(seeded_recs::columns::source.eq(movie_id))
        .load(connection)
        .unwrap()
        .iter()
        .map(|rec| get_movie_by_id(connection, rec.target.clone()).unwrap())
        .collect::<Vec<MovieWithGeneres>>()
}

// TODO: optimize
pub fn get_recs_from_associations(connection: &mut PgConnection, u_id: i32) -> Vec<MovieWithGeneres>{
    let logs = evidence_log::table
        .select(Log::as_select())
        .order(evidence_log::columns::created.desc())
        .filter(evidence_log::columns::user_id.eq(u_id))
        .limit(20)
        .load(connection)
        .unwrap();

    let mut movies: Vec<MovieWithGeneres> = vec![];
    for log in logs {
        let mut recs: Vec<MovieWithGeneres> = seeded_recs::table
            .select(SeededRec::as_select())
            .filter(seeded_recs::columns::source.eq(log.content_id))
            .load(connection)
            .unwrap()
            .iter()
            .map(|rec| get_movie_by_id(connection, rec.target.clone()).unwrap())
            .collect::<Vec<MovieWithGeneres>>();
        movies.append(&mut recs);
    }

    movies.into_iter().unique().collect()
}
