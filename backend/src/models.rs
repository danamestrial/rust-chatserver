use super::schema::posts;
use diesel::{Queryable, Insertable};
use serde::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub room: String,
    pub userid: i64,
    pub message: String,
}