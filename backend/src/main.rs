#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "pinged"
}

#[post("/login", format = "json", data = "<logininfo>")]
fn login(logininfo: Json<LoginInfo>) -> Json<bool>{
    println!("{:?}",logininfo);

    //get password from database using username
    Json(true)
}

// Json Format
// {
//     "username": "airbus",
//     "password": "yes1234"
// }

#[derive(Deserialize, Debug)]
struct LoginInfo {
    username: String,
    password: String,
}

#[launch]
fn rocket() ->  _ {
    rocket::build().mount("/api", routes![
        index,
        login,
        ])
}
