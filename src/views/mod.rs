use actix_web::web;

mod auth;
mod path;
mod users;
mod analytics;
mod movies;
mod ratings;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    users::user_factory(app);
    analytics::analytics_factory(app);
    movies::movie_factory(app);
    ratings::ratings_factory(app);
}
