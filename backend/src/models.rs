use super::schema::posts;
use diesel::{Queryable, Insertable};
use serde::*;
use rocket::FromForm;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub rooms: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub rooms: String,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, FromForm ,Serialize, Deserialize)]
pub struct Message {
    pub room: String,
    pub username: String,
    pub message: String,
}

#[derive(Debug, Clone, FromForm ,Serialize, Deserialize)]
pub struct Who {
    pub username: String,
    pub status: bool,
    pub rooms: String,
}

#[derive(Debug, Clone, FromForm ,Serialize, Deserialize)]
pub struct AddRoom {
    pub username: String,
    pub room: String,
}