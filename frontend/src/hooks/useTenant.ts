import { useEffect, useState } from 'react';
import { getCurrentTenant } from '../api/tenant';

export function useTenant() {
  const [tenantName, setTenantName] = useState<string | null>(null);

  useEffect(() => {
    getCurrentTenant()
      .then(t => setTenantName(t.tenant_name))
      .catch(() => {});
  }, []);

  return tenantName;
}
