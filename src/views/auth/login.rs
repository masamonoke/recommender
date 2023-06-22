use actix_web::{HttpResponse, web};
use crate::diesel;
use diesel::prelude::*;
use log;
use crate::database::establish_connection;
use crate::model::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users;
use crate::auth::jwt::JwtToken;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let mut connection = establish_connection();
    let users = users::table
        .filter(users::columns::name.eq(username.as_str()))
        .load::<User>(&mut connection)
        .unwrap();
    
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        log::error!("Multiple users have the username: {}", credentials.username.clone());
        return HttpResponse::Conflict().await.unwrap()
    }

    match users[0].clone().verify(password) {
        true => {
            let token = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok().header("token", token).await.unwrap()
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}
