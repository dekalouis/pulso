const API_BASE = 'http://localhost:3000';

export const getApiKey = () => localStorage.getItem('pulso_api_key');
export const setApiKey = (key: string) => localStorage.setItem('pulso_api_key', key);
export const clearApiKey = () => localStorage.removeItem('pulso_api_key');

export async function apiFetch<T>(path: string, options?: RequestInit): Promise<T> {
  const key = getApiKey();
  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(key ? { 'x-api-key': key } : {}), 
      ...options?.headers,
    },
  });
  if (!res.ok) throw new Error(`${res.status}`);

  return res.json() as Promise<T>;
}
