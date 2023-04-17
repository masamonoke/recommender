#[macro_use] extern crate diesel;
use actix_service::Service;
use actix_web::{App, HttpServer, HttpResponse};
use futures::future::{ok, Either};
use actix_cors::Cors;

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
        
        let cors = Cors::permissive();
        // let cors = Cors::default()
        //       .allowed_origin("http://localhost:8080")
        //       .allowed_methods(vec!["GET", "POST"])
        //       .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //       .allowed_header(http::header::CONTENT_TYPE)
        //       .max_age(3600);

        let app = App::new()
            .wrap_fn(|req, routing| {
                let req_url = String::from(req.uri().path().clone());
                log::info!("Handling request: {}", req_url);

                let mut passed = true;
                if req.path().contains("/analytics") {
                    match auth::process_token(&req) {
                        Err(_) => passed = false,
                        _ => ()
                    }
                }

                let result = match passed {
                    true => Either::Left(routing.call(req)),
                    false => Either::Right(
                        ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
                    )
                };

                async move {
                    let result = result.await?;
                    log::info!("{} -> {}", req_url, &result.status());
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors);
            
        return app;
    }).bind("127.0.0.1:3000")?.run().await
}
