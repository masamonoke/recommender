use actix_web::{web, Responder, HttpRequest};
use crate::{database::establish_connection, service::recommender};

use super::path::Path;


pub fn recommender_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/recommender"), backend: false};
    app.route(
        &base_path.define(String::from("/chart")), 
        web::get().to(chart)
    );
}

async fn chart(_req: HttpRequest) ->  impl Responder {
    let mut connection = establish_connection();
    web::Json(recommender::chart(&mut connection))
}
