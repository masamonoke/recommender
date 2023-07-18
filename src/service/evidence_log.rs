use diesel::{ExpressionMethods, RunQueryDsl, PgConnection};
use diesel::prelude::*;

use crate::{model::evidence::{Log, NewLog}, schema::evidence_log};

pub fn get_all_evidences(connection: &mut PgConnection) -> Vec<Log> {
   let evidences = evidence_log::table
       .order(evidence_log::columns::id.asc())
       .load::<Log>(connection)
       .unwrap();
   evidences
}

pub fn save_evidence(connection: &mut PgConnection, value: NewLog) -> Option<Log> {
    diesel::insert_into(evidence_log::table)
        .values(&value)
        .returning(Log::as_returning())
        .get_result(connection)
        .ok()
}
