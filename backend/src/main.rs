// entry point to start the server

mod models;
mod handlers;
mod middleware;
mod routes;
mod db;
mod state;
mod services;

use axum::{routing::get, Router};
use db::redis::create_redis_pool;
use handlers::events::health_check;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{CorsLayer, Any};
// use axum::middleware as axum_middleware;
// use middleware::auth::require_api_key;
use crate::state::AppState;
use axum::http::Method;

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

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = create_redis_pool(&redis_url).await;

    let state = AppState { pool, redis };

    let poll_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            let mut redis = poll_state.redis.clone();
            if let Err(e) = services::alerts::evaluate_rules(&poll_state.pool, &mut redis).await {
                eprintln!("Alert evaluation error: {}", e);
            }
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        // .route("/events", post(create_event).get(events))
        // .route_layer(axum_middleware::from_fn_with_state(pool.clone(), require_api_key))
        .merge(routes::event::routes(state.clone()))
        .merge(routes::tenant::routes(state.clone()))
        .merge(routes::alerts::routes(state.clone()))
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
