use actix_web::web;
use super::path::Path;

mod login;
mod logout;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path = Path { prefix: String::from("/auth"), backend: false };
    app.route(
        &base_path.define(String::from("/login")),
        web::post().to(login::login)
    );
    app.route(
        &base_path.define(String::from("/logout")),
        web::post().to(logout::logout)
    );
}
