use super::schema::posts;
use diesel::{Queryable, Insertable};
use serde::*;
use rocket::FromForm;

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

#[derive(Debug, Clone, FromForm ,Serialize, Deserialize)]
pub struct Message {
    pub room: String,
    pub userid: i32,
    pub message: String,
}