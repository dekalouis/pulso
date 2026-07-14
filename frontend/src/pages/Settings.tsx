import { useState } from 'react';
import { getApiKey, clearApiKey } from '../api/client';
import { useNavigate } from 'react-router-dom';
import { Check, Copy } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { Button } from '../components/ui/button';

export default function Settings() {
  const navigate = useNavigate();
  const key = getApiKey();
  const [copied, setCopied] = useState(false);

  function logout() {
    clearApiKey();
    navigate('/login');
  }

  function copyKey() {
    if (!key) return;
    navigator.clipboard.writeText(key);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  }

  return (
    <div className="flex flex-col gap-6">
      <h1 className="font-display text-2xl tracking-tight text-foreground">Settings</h1>
      <Card className="max-w-lg">
        <CardHeader>
          <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
            API key
          </CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <div className="flex items-center gap-2">
            <code className="flex-1 truncate rounded-md border border-border bg-muted px-3 py-2 font-mono text-xs text-foreground">
              {key}
            </code>
            <Button variant="outline" size="icon" onClick={copyKey} aria-label="Copy API key">
              {copied ? <Check size={14} className="text-primary" /> : <Copy size={14} />}
            </Button>
          </div>
          <Button variant="destructive" onClick={logout} className="w-fit">Sign out</Button>
        </CardContent>
      </Card>
    </div>
  );
}
