use actix_web::{HttpRequest, Responder, web};
use log::{error, info};
use crate::{database::establish_connection, service::movie::{get_limited_movies, get_movie_by_id}};
use qstring::QString;

const DEFAULT_LIMIT: i64 = 100;

pub async fn get(req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();

    let params = req.query_string();
    let qs = QString::from(params);
    let limit = qs.get("limit");
    let movies = match limit {
        Some(limit) => {
            info!("Handling limited movies request");
            let limit = match limit.parse::<i64>() {
                Ok(num) => num,
                Err(_) => {
                    error!("Wrong limit passed to 'movies/limit={}'", limit);
                    DEFAULT_LIMIT
                }
            };
            get_limited_movies(&mut connection, limit)
        },
        None => {
            info!("Handling default limited = {} movies request", DEFAULT_LIMIT);
            get_limited_movies(&mut connection, DEFAULT_LIMIT)
        }
    };
 
    web::Json(movies)
}

pub async fn get_by_id(req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    let id = req.match_info().get("movie_id").unwrap();
    let movie = get_movie_by_id(&mut connection, id.to_string());
    web::Json(movie)
}

