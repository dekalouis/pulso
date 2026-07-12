-- Add migration script here
CREATE TABLE alert_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  rule_id UUID NOT NULL REFERENCES alert_rules(id),
  tenant_id TEXT NOT NULL, 
  event_type TEXT NOT NULL, 
  rule_condition TEXT NOT NULL, 
  threshold INTEGER NOT NULL, 
  value_at_trigger INTEGER NOT NULL, 
  triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
  resolved_at TIMESTAMPTZ
);
