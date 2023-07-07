use actix_web::{Responder, web};

use crate::{database::establish_connection, model::evidence::NewLog, service::evidence_log};

pub async fn save_evidence(value: web::Json<NewLog>) -> impl Responder {
    let mut connection = establish_connection();
    let result = evidence_log::save_evidence(&mut connection, value.0);
    web::Json(result)
}
