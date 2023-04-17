use actix_web::web;

mod auth;
mod path;
mod users;
mod analytics;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    users::user_factory(app);
    analytics::analytics_factory(app);
}
