#[macro_use]
extern crate rocket;

// use rocket::form::{self, Error};

mod users;

#[derive(FromForm)]
struct Password<'r> {
    #[field(name = "password")]
    value: &'r str,
    #[field(validate = eq(self.value))]
    #[field(validate = omits("no"))]
    confirm: &'r str,
}

#[get("/world")]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/users", routes![users::login, users::logout, users::get_user])
}
