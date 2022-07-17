use super::schema::posts;

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