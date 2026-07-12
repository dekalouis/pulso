import { apiFetch } from './client';

export type AlertRule = {
  id: string; 
  event_type: string;
  rule_conditioin: string;
  threshold: number;
  time_window: string;
  is_active: boolean;
  created_at: string;
};

export type ALertEvent = {
  id: string;
  rule_id: string;
  event_type: string;
  rule_conditioin: string;
  threshold: number;
  value_at_trigger: number;
  triggered_at: string;
  resolved_at: string | null;
};

export type CreateAlertRuleInput = {
  event_type: string; 
  rule_conditioin: string;
  threshold: number;
  time_window: string;
};

export const getAlertRules = () => apiFetch<AlertRule[]>('/alert-rules');
export const createAlertRule = (body: CreateAlertRuleInput) => apiFetch<AlertRule>('/alert-rules', { method: 'POST', body: JSON.stringify(body) });
export const deleteAlertRule = (id: string) => apiFetch<void>(`/alert-rules/${id}`, { method: 'DELETE' });
export const getAlertEvents = () => apiFetch<ALertEvent[]>('/alerts');
