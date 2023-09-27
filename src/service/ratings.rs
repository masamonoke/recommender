use diesel::query_dsl::methods::FilterDsl;
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

pub fn get_user_ratings(connection: &mut PgConnection, user_id: i32) -> Vec<Rating> {
    let ratings = ratings::table
        .order(ratings::id.asc())
        .filter(ratings::columns::user_id.eq(user_id))
        .load(connection)
        .unwrap();
    ratings
}
