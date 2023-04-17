use actix_web::dev::ServiceRequest;
use super::jwt;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn check_token(token: String) -> Result<String, &'static str> {
    let t = token.clone();
    match jwt::JwtToken::decode(token) {
        Ok(_) => {
            match check_expire(t) {
               true => Ok(String::from("passed")),
               false => Err("Token has expired")
            }
        },
        Err(message) => Err(message)
    }
}

pub fn extract_header_token(req: &ServiceRequest) -> Result<String, &'static str> {
    match req.headers().get("user-token") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_password) => Ok(String::from(processed_password)),
                Err(_) => Err("There was an error processing token")
            }
        },
        None => Err("There is no token")
    }
}

fn check_expire(token: String) -> bool {
    let token = jwt::JwtToken::decode(token).unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    log::info!("Token has expired");
    return now as i32 - token.created_date < 60;
}
