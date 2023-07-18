use actix_web::{web, Responder, HttpRequest};
use crate::{database::establish_connection, service::recommender::{self, get_associated_with_objects}};

use super::path::Path;


pub fn recommender_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/recommender"), backend: false};
    app.route(
        &base_path.define(String::from("/chart")), web::get().to(chart)
    );
    app.route(
        &base_path.define(String::from("/associated/{movie_id}")), web::get().to(associated_movies)
    );
}

async fn chart(_req: HttpRequest) ->  impl Responder {
    let mut connection = establish_connection();
    web::Json(recommender::chart(&mut connection))
}

async fn associated_movies(req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    let movie_id = req.match_info().get("movie_id").unwrap().to_string();
    let movies = get_associated_with_objects(&mut connection, movie_id);
    web::Json(movies)
}
