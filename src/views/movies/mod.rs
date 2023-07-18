use actix_web::web;
use super::path::Path;

mod get;

pub fn movie_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/movie"), backend: false};
    app.route(
        &base_path.prefix, 
        web::get().to(get::get)
    );
    app.route(
        &base_path.define(String::from("/{movie_id}")), 
        web::get().to(get::get_by_id)
    );
}

