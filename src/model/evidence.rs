use serde::{Deserialize, Serialize};
use crate::schema::evidence_log;

#[derive(Debug, Queryable, Serialize, Identifiable, Selectable)]
#[diesel(table_name = evidence_log)]
pub struct Log {
    id: i32,
    created: chrono::NaiveDateTime,
    content_id: String,
    event: String,
    session_id: String,
    user_id: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = evidence_log)]
pub struct NewLog {
    created: chrono::NaiveDateTime,
    content_id: String,
    event: String,
    session_id: String,
    user_id: i32
}

impl NewLog {
    pub fn new(created: chrono::NaiveDateTime, content_id: String, event: String, session_id: String, user_id: i32) -> NewLog {
        NewLog {created, content_id, event, session_id, user_id}
    }
}
