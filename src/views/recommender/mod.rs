use actix_web::{web::{self}, Responder, HttpRequest};
use log::{info, error};
use redis::{JsonCommands, RedisResult};
use crate::{
    database::{establish_connection, establish_redis_connection},
    service::recommender::{self, get_associated_with_objects},
    model::movie::Movie
};

use super::path::Path;


pub fn recommender_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/recommender"), backend: false};
    app.route(
        &base_path.define(String::from("/chart")), web::get().to(chart)
    );
    app.route(
        &base_path.define(String::from("/associated/{movie_id}")),
        web::get().to(associated_movies)
    );
    app.route(
        &base_path.define(String::from("/associated/user/{user_id}")),
        web::get().to(get_recs_from_associations)
    );
    app.route(
        &base_path.define(String::from("/content_based/{user_id}")),
        web::get().to(get_recs_from_ratings)
    );

}


// TODO: move functions to separate module
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
    // TODO: test unwrap
    let movie_id = req.match_info().get("movie_id").unwrap().to_string();
    let movies = get_associated_with_objects(&mut connection, movie_id);
    web::Json(movies)
}

// TODO: make jwt check before proceed
async fn get_recs_from_associations(req: HttpRequest) -> impl Responder {
    let mut conn = establish_connection();
    let user_id = req.match_info().get("user_id").unwrap().parse::<i32>();
    let movies = match user_id {
        Ok(user_id) => recommender::get_recs_from_associations(&mut conn, user_id),
        Err(e) => {
            error!("user_id parse error: {}", e);
            vec![]
        }
    };

    web::Json(movies)
}

async fn get_recs_from_ratings(req: HttpRequest) -> impl Responder {
    let mut conn = establish_connection();
    let user_id = req.match_info().get("user_id").unwrap().parse::<i32>();
    let movies = match user_id {
        Ok(user_id) => recommender::get_recs_from_ratings(&mut conn, user_id),
        Err(e) => {
            error!("user_id parse_error: {}", e);
            vec![]
        }
    };
    web::Json(movies)
}
