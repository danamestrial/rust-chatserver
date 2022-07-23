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
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn authenticate<'a>(conn: &PgConnection, other_username: &'a String, other_password: &'a String)
-> bool {
    let user_password: String = posts
                        .filter(username.eq(other_username))
                        .select(password)
                        .first(conn)
                        .expect("Error saving new post");

    let valid = verify(other_password, &user_password);

    valid.unwrap()
}

pub fn add_user<'a>(conn: &PgConnection, other_username: &'a str, other_password: &'a str)
-> Post {
    use schema::posts;

    let new_user = NewUser {
        username: other_username,
        password: &hash(other_password, DEFAULT_COST).unwrap(),
    };

    diesel::insert_into(posts::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

// pub fn findUserByUsername<'a>(conn: &PgConnection, other_username: &'a str)
// -> bool {
//     use diesel::pg::expression::dsl::*;
//     let result = posts.select(username).filter(username.eq(any(other_username))).load::<QueryId>(conn);
//     println!("{:?} - result", result);
//     result.unwrap() != Vec::new()
// }