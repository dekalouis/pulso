import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { setApiKey } from '../api/client';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import PulseLine from '../components/PulseLine';

export default function Login() {
  const [key, setKey] = useState('');
  const navigate = useNavigate();

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!key.trim()) return;
    setApiKey(key.trim());
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

        <Card className="mt-10 w-full max-w-sm text-left">
          <CardHeader>
            <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
              Connect
            </CardTitle>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleSubmit} className="flex flex-col gap-3">
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
    </div>
  );
}
