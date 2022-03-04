mod structs;
mod utils;

use redis::AsyncCommands;
use rocket::serde::json::{serde_json::json, Json};

#[get("/sha256?<id>")]
pub async fn get_sha(id: &str) -> String {
    let mut con = utils::get_connection().await;
    let message = con.get::<_, String>(id).await;
    let x = match message {
        Ok(message) => json!({
           "status":  "success",
           "id":      id,
           "message": message,
        }),
        Err(e) => json!({
            "status":  "error",
            "id":      id,
            "error": e.to_string(),
        }),
    };
    x.to_string()
}

#[post("/sha256", format = "json", data = "<body>")]
pub async fn post_sha(body: Json<structs::Body>) -> String {
    let body = body.into_inner();
    let mut con = utils::get_connection().await;
    let message = &body.message;
    let sha = sha256::digest(message);
    con.set::<_, _, String>(&sha, &message).await.unwrap();
    json!({
        "status":  "success",
        "id":      &sha,
        "message": &message,
    })
    .to_string()
}
