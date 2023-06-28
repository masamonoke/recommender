use actix_web::web;
use super::path::Path;

mod get;

pub fn ratings_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/ratings"), backend: true};
    app.route(
        &base_path.prefix, 
        web::get().to(get::get)
    );
}

