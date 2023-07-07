use serde::{Serialize};

use crate::schema::ratings;

#[derive(Debug, Queryable, Clone, Identifiable, Serialize)]
pub struct Rating {
    id: i32,
    user_id: String,
    movie_id: String,
    rating: i32,
    rating_timestamp: chrono::NaiveDateTime,
    rating_type: String
}
