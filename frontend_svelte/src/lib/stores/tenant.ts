import { writable } from 'svelte/store';
import { getCurrentTenant } from '../api/tenant';

export const tenantName = writable<string | null>(null);

let loaded = false;

export function loadTenant() {
	if (loaded) return;
	loaded = true;
	getCurrentTenant()
		.then((t) => tenantName.set(t.tenant_name))
		.catch(() => {});
}
