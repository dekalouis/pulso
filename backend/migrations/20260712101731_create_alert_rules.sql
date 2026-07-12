-- Add migration script here

CREATE TABLE alert_rules (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  tenant_id TEXT NOT NULL,
  event_type TEXT NOT NULL, 
  rule_condition TEXT NOT NULL CHECK (rule_condition IN ('above', 'below')),
  threshold INTEGER NOT NULL, 
  time_window TEXT NOT NULL CHECK (time_window IN ('1m', '5m', '1hr')),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
