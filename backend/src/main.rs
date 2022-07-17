#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "pinged"
}

#[post("/login", format = "json", data = "<logininfo>")]
fn login(logininfo: Json<UserInfo>) -> Json<bool>{
    println!("{:?}",logininfo);

    //get password from database using username
    Json(true)
}

// Json Format
// {
//     "username": "airbus",
//     "password": "yes1234"
// }

#[post("/register", format = "json", data = "<regisinfo>")]
fn register(regisinfo: Json<UserInfo>) -> Json<String> {
    use rust_chatserver::*;

    let connection = establish_connection();
    let user = add_user(&connection, &regisinfo.username, &regisinfo.password);

    Json(format!("Username: {} logged in", user.username))
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    username: String,
    password: String,
}

#[launch]
fn rocket() ->  _ {
    rocket::build().mount("/api", routes![
        index,
        login,
        register,
        ])
}
