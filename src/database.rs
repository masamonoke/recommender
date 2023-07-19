use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use redis;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_redis_connection() -> redis::Connection {
    let url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(url.clone()).expect("Cannot aquire redis client");
    client.get_connection().unwrap_or_else(|_| panic!("Error connecting to {}", url))
}
