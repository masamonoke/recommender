use std::collections::HashMap;

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use diesel::{ExpressionMethods, RunQueryDsl, PgConnection};
use diesel::prelude::*;

use crate::model::movie::MovieWithGeneres;
use crate::model::recs::{SeededRec, Similarity};
use crate::model::{movie::Movie, evidence::Log};
use crate::schema::evidence_log::{dsl::*, self};
use crate::schema::{movies, seeded_recs};
use crate::service::ratings;

use super::movie::{get_movie_by_id, get_movies_with_genres_from_movies_list};
use super::similarity::get_similarities_from_movie_id_targets;

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

struct Rec {
    prediction: f32,
    similiar_movies: Vec<MovieWithGeneres>
}

impl Eq for Rec {}

impl PartialEq for Rec {
    fn eq(&self, other: &Self) -> bool {
        self.prediction == other.prediction
    }
}

impl PartialOrd for Rec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.prediction.partial_cmp(&other.prediction)
    }
}

impl std::cmp::Ord for Rec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prediction.total_cmp(&other.prediction)
    }
}

// TODO: very slow, optimize
pub fn get_recs_from_ratings(connection: &mut PgConnection, u_id: i32) -> Vec<MovieWithGeneres>{
    let ratings = ratings::get_user_ratings(connection, u_id);
    let mut movies: Vec<MovieWithGeneres> = vec![];
    if ratings.len() == 0 {
        return movies;
    }
    let movies_map  = ratings.iter()
        .map(|rating| (&rating.movie_id, &rating.rating))
        .collect::<HashMap<&String, &BigDecimal>>();

    let user_mean_rating = movies_map
        .values()
        .map(|v| v.clone())
        .sum::<BigDecimal>() / BigDecimal::from_usize(movies_map.len()).unwrap();

    let candidate_movies = get_similarities_from_movie_id_targets(connection, movies_map.keys().map(|k| k.clone().to_owned()).collect());
    let neighborhood_size = 15;
    let mut recs = vec![];
    for candidate in &candidate_movies {
        let target = candidate.target.clone();
        let mut pre = 0.0;
        let mut sim_sum = 0.0;
        let rated_items: Vec<&Similarity> = candidate_movies
            .iter()
            .filter(|c| c.target == target)
            .take(neighborhood_size)
            .collect();
        if rated_items.len() > 1 {
            for item in &rated_items {
                let r = movies_map.get(&item.source).unwrap().to_owned() - &user_mean_rating;
                pre += (&item.sim * r).to_f32().unwrap();
                sim_sum += &item.sim.to_f32().unwrap();
            }
            if sim_sum > 0.0 {
                let rec = Rec {
                    prediction: user_mean_rating.to_f32().unwrap() + pre / sim_sum,
                    similiar_movies: rated_items.iter().map(|m| get_movie_by_id(connection, m.source.clone()).unwrap()).collect()
                };
                recs.push(rec);
            }
        }
    }

    recs.sort();
    for r in recs {
        let mut m = r.similiar_movies.clone();
        movies.append(&mut m);
    }
    movies.into_iter().unique().collect()
}
