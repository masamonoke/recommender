use actix_web::{HttpRequest, Responder, web};
use crate::{database::establish_connection, service::movie::get_all_movies};

pub async fn get(_req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    let movies = get_all_movies(&mut connection); 
    web::Json(movies)
}
