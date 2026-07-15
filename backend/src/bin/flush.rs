// wipes event/alert data for a fresh demo state, keeps api_keys (tenants) intact

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::query("TRUNCATE TABLE alert_events, alert_rules, events")
        .execute(&pool)
        .await
        .expect("Failed to truncate tables");
    println!("Truncated events, alert_rules, alert_events (api_keys untouched)");

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to connect to Redis");
    let _: () = redis::cmd("FLUSHDB").query_async(&mut conn).await.expect("Failed to flush Redis");
    println!("Flushed Redis");
}
