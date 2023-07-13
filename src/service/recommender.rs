
use diesel::{ExpressionMethods, RunQueryDsl, PgConnection};
use diesel::prelude::*;

use crate::model::{movie::Movie, evidence::Log};
use crate::schema::evidence_log::{dsl::*, self};
use crate::schema::movies;

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
        // TODO: add match
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

pub fn chart(connection: &mut PgConnection) -> Vec<Movie> {
    // TODO: not yet found reason why this doesn't work and for now use workaround below
    // let res: Vec<ChartedLog> = sql_query("
    //     SELECT content_id, movies.title, count(*) as sold
    //     FROM evidence_log
    //     JOIN movies ON evidence_log.content_id = movies.movie_id
    //     WHERE event like 'buy'
    //     GROUP BY content_id, movies.title
    //     ORDER BY sold DESC LIMIT 10;
    // ")
    //     .get_result(connection)
    //     .unwrap();
 
    let all_movies = movies::table
        .select(Movie::as_select())
        .load(connection)
        .unwrap();

    let logs = evidence_log::table
        .select(Log::as_select())
        .filter(event.eq("buy"))
        .load(connection)
        .unwrap();

    let mut counted_movies = vec![];
    for movie in all_movies {
        let movie_buys_count = logs.iter().filter(|l| l.content_id == movie.movie_id).count();
        let value = MovieCounted { movie, count: movie_buys_count };
        counted_movies.push(value);
    }
    counted_movies.sort();

    let top_movies: Vec<Movie> = counted_movies.iter().map(|v| v.movie.clone()).take(10).collect();

    top_movies
}
