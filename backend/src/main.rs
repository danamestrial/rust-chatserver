#[macro_use] extern crate rocket;

use rocket::response::Redirect;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/* This section is for authetication system
 * Uncomment when you start to implement this

#[get("/login")]
fn login() -> Template { /* .. */ }

#[get("/admin")]
fn admin_panel(admin: AdminUser) -> &'static str {
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin", rank = 2)]
fn admin_panel_user(user: User) -> &'static str {
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect {
    Redirect::to(uri!(login))
}

*/

#[launch]
fn rocket() ->  _ {
    rocket::build().mount("/", routes![index])
}
