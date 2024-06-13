
use axum::{
    routing::get,
    Json, Router, Server,
};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    Server::bind(&addr).serve(app.into_make_service()).await.expect("Failed to start server");
    println!("Hello, world!");
}

#[derive(serde::Serialize)]
struct Message {
    message: String
}

async fn handler() -> Json<Message> {
    Json(Message { message: String::from("Hello, World") })
}