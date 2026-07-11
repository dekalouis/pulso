use redis::aio::ConnectionManager;
use std::collections::HashMap;
use uuid::Uuid;
use redis::AsyncCommands;

pub struct WindowCounts {
    pub one_min: i64,
    // TODO pub five_min: i64,
    // TODO pub one_hour: i64,
}

pub async fn record_event(
    redis: &mut ConnectionManager,
    tenant_id: &str,
    event_type: &str,
) -> Result<(), redis::RedisError> {
    let key = format!("metrics:{}:{}", tenant_id, event_type);
    let now_ms = chrono::Utc::now().timestamp_millis() as f64;
    let one_hour_ago_ms = now_ms - 3_600_000.0;
    let member = Uuid::new_v4().to_string();

    let _: () = redis.zadd(&key, &member, now_ms).await?;
    // let _: () = redis.zremrangebyscore(&key, 0, one_hour_ago_ms).await?;
    let _: () = redis::cmd("ZREMRANGEBYSCORE")
        .arg(&key)
        .arg(0i64)
        .arg(one_hour_ago_ms)
        .query_async(redis)
        .await?;
    Ok(())
}

pub async fn get_counts(
    redis: &mut ConnectionManager,
    tenant_id: &str,
) -> Result<HashMap<String, WindowCounts>, redis::RedisError> {
    let pattern = format!("metrics:{}:*", tenant_id);
    let keys: Vec<String> = redis.keys(&pattern).await?;

    let now_ms = chrono::Utc::now().timestamp_millis();
    let one_min_ago_ms = now_ms - 60_000;
    let prefix = format!("metrics:{}:", tenant_id);

    let mut counts = HashMap::new();

    for key in keys {
        let event_type = key.trim_start_matches(&prefix).to_string();
        let count: i64 = redis.zcount(&key, one_min_ago_ms, now_ms).await?;
        counts.insert(event_type, WindowCounts { one_min: count });
    }
    Ok(counts)
}
