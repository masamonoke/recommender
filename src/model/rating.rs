use serde::{Serialize, ser::SerializeStruct};

use crate::schema::ratings;

#[derive(Debug, Queryable, Clone, Identifiable)]
pub struct Rating {
    id: i32,
    user_id: String,
    movie_id: String,
    rating: i32,
    rating_timestamp: chrono::NaiveDateTime,
    rating_type: String
}

impl Serialize for Rating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Rating", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("movie_id", &self.movie_id)?;
        state.serialize_field("rating", &self.rating)?;
        state.serialize_field("rating_timestamp", &self.rating_timestamp.to_string())?;
        state.serialize_field("rating_type", &self.rating_type)?;
        state.end()
    }
}
