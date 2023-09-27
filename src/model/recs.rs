use bigdecimal::BigDecimal;
use crate::schema::{seeded_recs, similarity};
use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone, Identifiable, Selectable, PartialEq, Queryable)]
#[diesel(table_name = seeded_recs)]
pub struct SeededRec {
    id: i32,
    created: chrono::NaiveDateTime,
    source: String,
    pub target: String,
    support: BigDecimal,
    confidence: BigDecimal,
}

impl Serialize for SeededRec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("SeededRec", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("created", &self.created)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("target", &self.target)?;
        state.serialize_field("support", &self.support.to_string())?;
        state.serialize_field("confidence", &self.confidence.to_string())?;
        state.end()
    }
}

#[derive(Debug, Clone, Identifiable, Selectable, PartialEq, Queryable)]
#[diesel(table_name = similarity)]
pub struct Similarity {
    id: i32,
    created: chrono::NaiveDateTime,
    pub source: String,
    pub target: String,
    pub sim: BigDecimal
}

impl Serialize for Similarity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("SeededRec", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("created", &self.created)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("target", &self.target)?;
        state.serialize_field("similarity", &self.sim.to_string())?;
        state.end()
    }
}
