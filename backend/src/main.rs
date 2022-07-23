#[macro_use] extern crate rocket;

use rust_chatserver::*;
pub mod models;
use models::*;
extern crate base64;

use base64::{encode, decode};

use rocket::serde::json::Json;
use rocket::{State, Shutdown};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};

#[get("/")]
fn index() -> &'static str {
    "pinged"
}

#[post("/login", format = "json", data = "<logininfo>")]
fn login(cookies: &CookieJar, logininfo: Json<UserInfo>) -> Json<bool>{
    println!("{:?}",logininfo);
    let connection = establish_connection();
    
    let now = (&logininfo.username).clone();
    //Authenticate
    let access = authenticate(&connection, &logininfo.username, &logininfo.password);
    if access {
        cookies.add(Cookie::new("current_user", now));
    };

    Json(access)
}

#[post("/register", format = "json", data = "<regisinfo>")]
fn register(regisinfo: Json<UserInfo>) -> Json<String> {
    let connection = establish_connection();
    let user = add_user(&connection, &regisinfo.username, &regisinfo.password);

    Json(format!("Username: {} logged in", user.username))
}


#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

/*
Recieve message from frontend then distribute it
*/
#[post("/message", data = "<msginfo>")]
fn post(msginfo: Json<Message>, queue: &State<Sender<Message>>) {
    let _y = queue.send(msginfo.into_inner());
    // Json("Binged".to_string())
}

#[get("/whoami")]
fn whoami(cookies: &CookieJar) -> Json<Who>{
    match cookies.get("current_user") {
        Some(x) => {
            println!("{:?}",x.value());
            let who = Who{
                username: x.value().to_string(),
                status: true,
            };
            return Json(who);
        }
        None => {
            let who = Who{
                username: "".to_string(),
                status: false,
            };
            return Json(who);
        }
    }
}

/* Credit to Stackoverflow for cors headers responses
https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
*/

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() ->  _ {
    rocket::build()
        .attach(CORS)
        .manage(channel::<Message>(1024).0)
        .mount("/api", routes![
        index,
        login,
        register,
        post,
        events,
        whoami,
        ])
}
