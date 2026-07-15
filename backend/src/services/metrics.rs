use redis::aio::ConnectionManager;
use std::collections::HashMap;
use uuid::Uuid;
use redis::AsyncCommands;

pub struct WindowCounts {
    pub five_min: i64,
    pub fifteen_min: i64,
    pub one_hour: i64,
    pub one_day: i64,
    /// (bucket_start_ms, count) for 60 one-minute buckets covering the last hour
    pub series: Vec<(i64, i64)>,
}

const SERIES_BUCKETS: i64 = 60;
const SERIES_BUCKET_MS: i64 = 60_000;

pub async fn record_event(
    redis: &mut ConnectionManager,
    tenant_id: &str,
    event_type: &str,
) -> Result<(), redis::RedisError> {
    let key = format!("metrics:{}:{}", tenant_id, event_type);
    let now_ms = chrono::Utc::now().timestamp_millis() as f64;
    let one_day_ago_ms = now_ms - 86_400_000.0;
    let member = Uuid::new_v4().to_string();

    let _: () = redis.zadd(&key, &member, now_ms).await?;
    let _: () = redis::cmd("ZREMRANGEBYSCORE")
        .arg(&key)
        .arg(0i64)
        .arg(one_day_ago_ms)
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
    let five_min_ago_ms = now_ms - 300_000;
    let fifteen_min_ago_ms = now_ms - 900_000;
    let one_hour_ago_ms = now_ms - 3_600_000;
    let one_day_ago_ms = now_ms - 86_400_000;
    let prefix = format!("metrics:{}:", tenant_id);

    let mut counts = HashMap::new();

    for key in keys {
        let event_type = key.trim_start_matches(&prefix).to_string();
        let count_five_min: i64 = redis.zcount(&key, five_min_ago_ms, now_ms).await?;
        let count_fifteen_min: i64 = redis.zcount(&key, fifteen_min_ago_ms, now_ms).await?;
        let count_one_hour: i64 = redis.zcount(&key, one_hour_ago_ms, now_ms).await?;
        let count_one_day: i64 = redis.zcount(&key, one_day_ago_ms, now_ms).await?;

        let members: Vec<(String, f64)> = redis
            .zrangebyscore_withscores(&key, one_hour_ago_ms, now_ms)
            .await?;
        let mut buckets = vec![0i64; SERIES_BUCKETS as usize];
        for (_, score) in members {
            let idx = ((score as i64 - one_hour_ago_ms) / SERIES_BUCKET_MS)
                .clamp(0, SERIES_BUCKETS - 1) as usize;
            buckets[idx] += 1;
        }
        let series = buckets
            .into_iter()
            .enumerate()
            .map(|(i, count)| (one_hour_ago_ms + i as i64 * SERIES_BUCKET_MS, count))
            .collect();

        counts.insert(event_type, WindowCounts {
            five_min: count_five_min,
            fifteen_min: count_fifteen_min,
            one_hour: count_one_hour,
            one_day: count_one_day,
            series,
        });
    }
    Ok(counts)
}
