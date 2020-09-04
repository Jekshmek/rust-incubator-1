#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("Can`t connect to {}", &db_url))
}
