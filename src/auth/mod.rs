use actix_web::dev::ServiceRequest;

pub mod jwt;
mod processes;

pub fn process_token(req: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(req) {
        Ok(token) => {
            match processes::check_token(token) {
                Ok(token) => Ok(token),
                Err(message) => Err(message)
            }
        },
        Err(message) => Err(message)
    }
}
