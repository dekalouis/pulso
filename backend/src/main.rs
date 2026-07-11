// entry point to start the server

mod models;
mod handlers;
mod middleware;

use axum::{routing::{get, post}, Router};
use handlers::events::{create_event, events, health_check};
use sqlx::postgres::PgPoolOptions;
use axum::middleware as axum_middleware;
use middleware::auth::require_api_key;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    let app = Router::new()
        .route("/events", post(create_event).get(events))
        .route_layer(axum_middleware::from_fn_with_state(pool.clone(), require_api_key))
        .route("/health", get(health_check))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
