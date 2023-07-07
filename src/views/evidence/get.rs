use actix_web::{Responder, web, HttpRequest};

use crate::{database::establish_connection, service::evidence_log::get_all_evidences};

pub async fn get(_req: HttpRequest) -> impl Responder {
    let mut connection = establish_connection();
    let evidences = get_all_evidences(&mut connection);
    web::Json(evidences)
}
