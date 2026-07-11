use redis::{Client, aio::ConnectionManager};

pub async fn create_redis_pool(redis_url: &str) -> ConnectionManager {
    let client = Client::open(redis_url).expect("Invalid Redis URL");
    ConnectionManager::new(client)
        .await
        .expect("Failed to connect to Redis")
}
