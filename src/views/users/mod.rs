use actix_web::web;
use super::path::Path;

mod create;

pub fn user_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/user"), backend: true};
    app.route(
        &base_path.define(String::from("/create")), 
        web::post().to(create::create)
    );
}
