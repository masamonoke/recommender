use actix_web::dev::ServiceRequest;
use super::jwt;

pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_) => Ok(String::from("passed")),
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
