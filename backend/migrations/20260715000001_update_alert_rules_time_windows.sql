-- Update time_window constraint to use new window values (5m, 15m, 1h, 24h)

ALTER TABLE alert_rules DROP CONSTRAINT IF EXISTS alert_rules_time_window_check;
ALTER TABLE alert_rules ADD CONSTRAINT alert_rules_time_window_check CHECK (time_window IN ('5m', '15m', '1h', '24h'));
