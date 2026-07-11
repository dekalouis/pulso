//connect db, and insert test api key 


use sha2::{Digest, Sha256};
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

    let raw_key = "test-key-123";
    let mut hasher = Sha256::new();
    hasher.update(raw_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    sqlx::query("INSERT INTO api_keys (key_hash, tenant_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(&key_hash)
        .bind("tenant-acme")
        .execute(&pool)
        .await
        .expect("Failed to seed api key");

    println!("Seeded API key for tenant: tenant-acme");
}
