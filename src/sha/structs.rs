use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Body {
    pub message: String,
}
