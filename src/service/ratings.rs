use diesel::{PgConnection, query_dsl::methods::OrderDsl, ExpressionMethods, RunQueryDsl};
use crate::model::rating::Rating;
use crate::schema::ratings;

pub fn get_all_ratings(connection: &mut PgConnection) -> Vec<Rating> {
    let ratings = ratings::table
        .order(ratings::columns::id.asc())
        .load::<Rating>(connection)
        .unwrap();
    ratings
}
