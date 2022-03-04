pub async fn get_connection() -> redis::aio::Connection {
    redis::Client::open("redis://127.0.0.1/").unwrap().get_async_connection().await.unwrap()
}
