import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Check, Copy } from 'lucide-react';
import { setApiKey } from '../api/client';
import { createTenant } from '../api/tenant';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import PulseLine from '../components/PulseLine';

export default function Login() {
  const [key, setKey] = useState('');
  const [tenantName, setTenantName] = useState('');
  const [creating, setCreating] = useState(false);
  const [createError, setCreateError] = useState<string | null>(null);
  const [createdKey, setCreatedKey] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const navigate = useNavigate();

  function handleConnect(e: React.FormEvent) {
    e.preventDefault();
    if (!key.trim()) return;
    setApiKey(key.trim());
    navigate('/');
  }

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    if (!tenantName.trim()) return;
    setCreating(true);
    setCreateError(null);
    try {
      const res = await createTenant(tenantName.trim());
      setCreatedKey(res.api_key);
    } catch {
      setCreateError('Could not create tenant. Try again.');
    } finally {
      setCreating(false);
    }
  }

  function copyKey() {
    if (!createdKey) return;
    navigator.clipboard.writeText(createdKey);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  }

  function continueToDashboard() {
    if (!createdKey) return;
    setApiKey(createdKey);
    navigate('/');
  }

  return (
    <div className="relative flex min-h-screen flex-col items-center justify-center overflow-hidden bg-background px-6">
      <PulseLine
        tiles={14}
        className="pointer-events-none absolute inset-x-0 top-1/2 h-48 w-full -translate-y-1/2 text-primary/15"
      />

      <div className="relative z-10 flex w-full flex-col items-center text-center">
        <div className="mb-3 flex items-center gap-2 font-mono text-xs tracking-[0.3em] text-muted-foreground uppercase">
          <span className="led-live h-1.5 w-1.5 rounded-full bg-primary" />
          Self-hosted signal
        </div>
        <h1 className="font-display text-6xl tracking-tight text-foreground sm:text-7xl">PULSO</h1>
        <p className="mt-3 max-w-sm text-sm text-muted-foreground">
          Real-time event metrics and threshold alerts for your own infrastructure.
        </p>

        {createdKey ? (
          <Card className="mt-10 w-full max-w-sm text-left">
            <CardHeader>
              <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
                Save your API key
              </CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-4">
              <p className="text-xs text-muted-foreground">
                This is shown once — store it somewhere safe. You'll need it to send events and to sign back in later.
              </p>
              <div className="flex items-center gap-2">
                <code className="flex-1 truncate rounded-md border border-border bg-muted px-3 py-2 font-mono text-xs text-foreground">
                  {createdKey}
                </code>
                <Button variant="outline" size="icon" onClick={copyKey} aria-label="Copy API key">
                  {copied ? <Check size={14} className="text-primary" /> : <Copy size={14} />}
                </Button>
              </div>
              <Button className="w-full" onClick={continueToDashboard}>Continue to dashboard</Button>
            </CardContent>
          </Card>
        ) : (
          <div className="mt-10 flex w-full max-w-sm flex-col gap-4">
            <Card className="text-left">
              <CardHeader>
                <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
                  Create new tenant
                </CardTitle>
              </CardHeader>
              <CardContent>
                <form onSubmit={handleCreate} className="flex flex-col gap-3">
                  <Input
                    placeholder="Tenant name"
                    value={tenantName}
                    onChange={e => setTenantName(e.target.value)}
                    className="font-mono"
                  />
                  {createError && <p className="text-xs text-destructive">{createError}</p>}
                  <Button type="submit" className="w-full" disabled={creating}>
                    {creating ? 'Creating…' : 'Create tenant'}
                  </Button>
                </form>
              </CardContent>
            </Card>

            <Card className="text-left">
              <CardHeader>
                <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
                  Connect
                </CardTitle>
              </CardHeader>
              <CardContent>
                <form onSubmit={handleConnect} className="flex flex-col gap-3">
                  <Input
                    placeholder="API key"
                    value={key}
                    onChange={e => setKey(e.target.value)}
                    className="font-mono"
                    autoFocus
                  />
                  <Button type="submit" className="w-full">Connect</Button>
                </form>
              </CardContent>
            </Card>
          </div>
        )}
      </div>
    </div>
  );
}
