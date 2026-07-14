import { useEffect, useState } from 'react';
import { ArrowUp, ArrowDown, Trash2 } from 'lucide-react';
import { getAlertRules, deleteAlertRule, getAlertEvents, createAlertRule } from '../api/alerts';
import type { AlertRule, AlertEvent, CreateAlertRuleInput } from '../api/alerts';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { Badge } from '../components/ui/badge';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '../components/ui/table';

const selectClass =
  'h-9 rounded-md border border-input bg-transparent px-2.5 text-sm text-foreground outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 dark:bg-input/30';

const th = 'font-mono text-[10px] tracking-widest text-muted-foreground uppercase';

function ConditionBadge({ condition }: { condition: string }) {
  const above = condition === 'above';
  return (
    <Badge variant="outline" className="gap-1 font-mono">
      {above ? <ArrowUp size={11} /> : <ArrowDown size={11} />}
      {condition}
    </Badge>
  );
}

export default function Alerts() {
  const [rules, setRules] = useState<AlertRule[]>([]);
  const [events, setEvents] = useState<AlertEvent[]>([]);
  const [form, setForm] = useState<CreateAlertRuleInput>({
    event_type: '', rule_condition: 'above', threshold: 1, time_window: '1m',
  });

  const load = () => {
    getAlertRules().then(setRules).catch(console.error);
    getAlertEvents().then(setEvents).catch(console.error);
  };

  useEffect(() => {
    load();
    const id = setInterval(load, 10_000);
    return () => clearInterval(id);
  }, []);

  async function handleDelete(id: string) {
    await deleteAlertRule(id);
    load();
  }

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    await createAlertRule(form);
    load();
  }

  return (
    <div className="flex flex-col gap-6">
      <h1 className="font-display text-2xl tracking-tight text-foreground">Alerts</h1>

      <Card>
        <CardHeader>
          <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
            New alert rule
          </CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleCreate} className="flex flex-wrap items-end gap-3">
            <div className="flex flex-col gap-1.5">
              <label className="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Event type</label>
              <Input
                value={form.event_type}
                onChange={e => setForm(f => ({ ...f, event_type: e.target.value }))}
                placeholder="checkout"
                className="w-36 font-mono"
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <label className="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Condition</label>
              <select
                value={form.rule_condition}
                onChange={e => setForm(f => ({ ...f, rule_condition: e.target.value }))}
                className={selectClass}
              >
                <option value="above">above</option>
                <option value="below">below</option>
              </select>
            </div>
            <div className="flex flex-col gap-1.5">
              <label className="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Threshold</label>
              <Input
                type="number"
                value={form.threshold}
                onChange={e => setForm(f => ({ ...f, threshold: Number(e.target.value) }))}
                className="w-24 font-mono"
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <label className="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Window</label>
              <select
                value={form.time_window}
                onChange={e => setForm(f => ({ ...f, time_window: e.target.value }))}
                className={selectClass}
              >
                <option value="1m">1 min</option>
                <option value="5m">5 min</option>
                <option value="1hr">1 hr</option>
              </select>
            </div>
            <Button type="submit">Create rule</Button>
          </form>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
            Active rules
          </CardTitle>
        </CardHeader>
        <CardContent>
          {rules.length === 0 ? (
            <p className="text-sm text-muted-foreground">No active rules yet.</p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className={th}>Event</TableHead>
                  <TableHead className={th}>Condition</TableHead>
                  <TableHead className={th}>Threshold</TableHead>
                  <TableHead className={th}>Window</TableHead>
                  <TableHead />
                </TableRow>
              </TableHeader>
              <TableBody>
                {rules.map(r => (
                  <TableRow key={r.id}>
                    <TableCell className="text-foreground">{r.event_type}</TableCell>
                    <TableCell><ConditionBadge condition={r.rule_condition} /></TableCell>
                    <TableCell className="font-mono">{r.threshold}</TableCell>
                    <TableCell className="font-mono text-muted-foreground">{r.time_window}</TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="ghost"
                        size="icon-sm"
                        onClick={() => handleDelete(r.id)}
                        className="text-muted-foreground hover:text-destructive"
                      >
                        <Trash2 size={14} />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="font-mono text-xs tracking-widest text-muted-foreground uppercase">
            Alert history
          </CardTitle>
        </CardHeader>
        <CardContent>
          {events.length === 0 ? (
            <p className="text-sm text-muted-foreground">No alerts triggered yet.</p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className={th}>Event</TableHead>
                  <TableHead className={th}>Condition</TableHead>
                  <TableHead className={th}>Value</TableHead>
                  <TableHead className={th}>Fired</TableHead>
                  <TableHead className={th}>Status</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {events.map(e => (
                  <TableRow key={e.id}>
                    <TableCell className="text-foreground">{e.event_type}</TableCell>
                    <TableCell className="font-mono text-muted-foreground">{e.rule_condition} {e.threshold}</TableCell>
                    <TableCell className="font-mono">{e.value_at_trigger}</TableCell>
                    <TableCell className="font-mono text-muted-foreground">{new Date(e.triggered_at).toLocaleString()}</TableCell>
                    <TableCell>
                      {e.resolved_at ? (
                        <Badge variant="secondary">Resolved</Badge>
                      ) : (
                        <Badge className="gap-1.5 border-transparent bg-amber/15 text-amber">
                          <span className="led-live h-1.5 w-1.5 rounded-full bg-amber" />
                          Firing
                        </Badge>
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
