use redis::aio::ConnectionManager;
use sqlx::PgPool;
use redis::AsyncCommands;


pub async fn evaluate_rules(
    pool: &PgPool,
    redis: &mut ConnectionManager,
) -> Result<(), Box<dyn std::error::Error>> {
    let rules = sqlx::query_as::<_, crate::models::alert::AlertRule>(
        "SELECT * FROM alert_rules WHERE is_active = TRUE",
        )
        .fetch_all(pool)
        .await?;

    for rule in rules {
        let window_ms: f64 = match rule.time_window.as_str() {
            "5m" => 300_000.0,
            "15m" => 900_000.0,
            "1h" => 3_600_000.0,
            "24h" => 86_400_000.0,
            _    => continue,
        };

        let now_ms = chrono::Utc::now().timestamp_millis() as f64;
        let since_ms = now_ms - window_ms;
        let key = format!("metrics:{}:{}", rule.tenant_id, rule.event_type);

        let count: i64 = redis.zcount(&key, since_ms, now_ms).await?;

        let condition_met = match rule.rule_condition.as_str() {
            "above" => count > rule.threshold as i64,
            "below" => count < rule.threshold as i64,
            _       => continue,
        };

        let open_alert = sqlx::query_scalar::<_, uuid::Uuid>(
            "SELECT id FROM alert_events WHERE rule_id = $1 AND resolved_at IS NULL LIMIT 1",
            )
            .bind(rule.id)
            .fetch_optional(pool)
            .await?;

        if condition_met && open_alert.is_none() {
            sqlx::query(
                "INSERT INTO alert_events (rule_id, tenant_id, event_type, rule_condition, threshold, value_at_trigger)
                VALUES ($1, $2, $3, $4, $5, $6)",
                )
                .bind(rule.id)
                .bind(&rule.tenant_id)
                .bind(&rule.event_type)
                .bind(&rule.rule_condition)
                .bind(&rule.threshold)
                .bind(count as i32)
                .execute(pool)
                .await?;

        } else if !condition_met && open_alert.is_some() {
            sqlx::query(
                "UPDATE alert_events SET resolved_at = NOW() WHERE id = $1",
                )
                .bind(open_alert.unwrap())
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}
