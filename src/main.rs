#[macro_use]
extern crate rocket;

mod sha;
mod users;

#[get("/")]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/", routes![sha::get_sha, sha::post_sha])
        .mount(
            "/users",
            routes![users::login, users::logout, users::get_user],
        )
}
