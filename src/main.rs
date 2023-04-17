#[macro_use] extern crate diesel;
use actix_service::Service;
use actix_web::{App, HttpServer};

mod views;
mod model;
mod schema;
mod database;
mod json_serialization;
mod auth;

extern crate dotenv;
use env_logger;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, routing| {
                let req_url = String::from(req.uri().path().clone());
                log::info!("Handling request: {}", req_url);
                routing.call(req)
            })
            .configure(views::views_factory);
        return app;
    }).bind("127.0.0.1:3000")?.run().await
}
