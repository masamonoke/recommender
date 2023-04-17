use actix_web::HttpResponse;

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("<h1>Logout page</h1>")
}

