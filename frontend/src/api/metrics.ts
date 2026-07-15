import { apiFetch } from './client';

export type MetricsResponse = {
  tenant_id: string;
  windows: Record<string, Record<string, number>>;
  series: Record<string, [number, number][]>;
}

export const getMetrics = () => apiFetch<MetricsResponse>('/metrics');

