use actix_web::{HttpRequest, Responder, web};
use crate::{database::establish_connection, service::ratings::get_all_ratings};

pub async fn get(_req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    let rating = get_all_ratings(&mut connection); 
    web::Json(rating)
}

