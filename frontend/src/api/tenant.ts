import { apiFetch } from './client';

export type TenantInfo = {
  tenant_id: string;
  tenant_name: string;
};

export type CreateTenantResponse = TenantInfo & {
  api_key: string;
};

export const createTenant = (tenantName: string) =>
  apiFetch<CreateTenantResponse>('/tenants', {
    method: 'POST',
    body: JSON.stringify({ tenant_name: tenantName }),
  });

export const getCurrentTenant = () => apiFetch<TenantInfo>('/tenant');
