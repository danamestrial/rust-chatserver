#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bcrypt;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use schema::posts::dsl::*;
use std::io;
use models::{Post, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn authenticate(other_username: String, other_password: String)
-> Result<String, io::Error> {
    panic!();
}

pub fn add_user<'a>(conn: &PgConnection, other_username: &'a str, other_password: &'a str)
-> Post {
    use schema::posts;
    use bcrypt::{DEFAULT_COST, hash, verify};

    let new_user = NewUser {
        username: other_username,
        password: &hash(other_password, DEFAULT_COST).unwrap(),
    };

    diesel::insert_into(posts::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}