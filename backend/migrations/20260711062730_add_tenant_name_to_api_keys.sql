-- Add migration script here
ALTER TABLE api_keys ADD COLUMN tenant_name TEXT NOT NULL DEFAULT '';
