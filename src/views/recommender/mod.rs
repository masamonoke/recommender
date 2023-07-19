use actix_web::{web::{self}, Responder, HttpRequest};
use log::{info, error};
use redis::{JsonCommands, RedisResult, FromRedisValue};
use crate::{database::{establish_connection, establish_redis_connection}, service::recommender::{self, get_associated_with_objects}, model::movie::{MovieWithGeneres, Movie}};

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
    let mut redis_connection = establish_redis_connection();
    let mut movies: Vec<Movie> = vec![];
    match redis_connection.json_get("chart", "$") {
        Ok(m) => {
            let json: String = m;
            info!("Got values from redis \n{:?}", json);
            let mut chars = json.chars();
            chars.next();
            chars.next_back();
            let json = chars.as_str();
            movies = serde_json::from_str(&json).unwrap();
        },
        Err(e) => {
            error!("Cannot get value chart\n{}", e);
        }
    };
    if movies.len() == 0 {
            info!("Chart isn't cached. Calculating...");
            let mut connection = establish_connection();
            movies = recommender::chart(&mut connection);
            let res: RedisResult<bool> = redis_connection.json_set("chart", "$", &movies);
            match res {
                Ok(_) => info!("Successfully saved chart to redis cache"),
                Err(e) => error!("Cannot save chart to redis cache {}", e)
            }
    }
    web::Json(movies)
}

async fn associated_movies(req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    let movie_id = req.match_info().get("movie_id").unwrap().to_string();
    let movies = get_associated_with_objects(&mut connection, movie_id);
    web::Json(movies)
}
