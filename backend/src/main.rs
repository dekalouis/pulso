// entry point to start the server

mod models;
mod handlers;

use axum::{routing::{get, post}, Router};
use handlers::events::{create_event, health_check};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/events", post(create_event))
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
