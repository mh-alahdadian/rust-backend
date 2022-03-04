mod structs;

#[get("/login")]
pub fn login() -> String {
    "Hello, world!".to_string()
}

#[get("/logout")]
pub fn logout() -> String {
    "Hello, world!".to_string()
}

#[get("/<id>")]
pub fn get_user(id: &str) -> String {
    format!("Hello, world! {id}")
}
