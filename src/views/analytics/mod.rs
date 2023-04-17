use actix_web::{HttpResponse, web};

pub async fn analytics() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("<h1>Analytics page</h1>")
}

pub fn analytics_factory(app: &mut web::ServiceConfig) {
    app.route(
        String::from("/analytics").as_str(),
        web::get().to(analytics)
    );
}


