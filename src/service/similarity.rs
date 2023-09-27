use diesel::prelude::*;

use crate::{schema::similarity, model::recs::Similarity};

// TODO: fix unwraps
pub fn get_similarity_from_movie_id_target(connection: &mut PgConnection, movie_id: String) -> Option<Similarity> {
    similarity::table
        .select(Similarity::as_select())
        .filter(similarity::columns::source.eq(movie_id))
        .order(similarity::sim)
        .load(connection)
        .unwrap()
        .last()
        .clone()
        .cloned()
}

// WARNING: slow
pub fn get_similarities_from_movie_id_targets(connection: &mut PgConnection, movie_ids: Vec<String>) -> Vec<Similarity> {
    let mut similarities: Vec<Similarity> = vec![];
    for id in movie_ids {
        match get_similarity_from_movie_id_target(connection, id) {
            Some(sim) => similarities.push(sim),
            None => continue
        }
    }
    return similarities;
}
