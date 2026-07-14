import { useEffect, useState } from 'react';
import { getMetrics, type MetricsResponse } from '../api/metrics';
import MetricsChart from '../components/MetricsChart';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';

export default function Dashboard() {
  const [data, setData] = useState<MetricsResponse | null>(null);

  useEffect(() => {
    const load = () => getMetrics().then(setData).catch(console.error);
    load();
    const id = setInterval(load, 10_000);
    return () => clearInterval(id);
  }, []);

  const windows = data?.windows ?? {};
  const eventTypes = Object.keys(windows);

  return (
    <div className="flex flex-col gap-6">
      <div className="flex items-center justify-between">
        <h1 className="font-display text-2xl tracking-tight text-foreground">Dashboard</h1>
        <div className="flex items-center gap-2 font-mono text-xs tracking-widest text-muted-foreground uppercase">
          <span className="led-live h-1.5 w-1.5 rounded-full bg-primary" />
          Live
        </div>
      </div>

      {eventTypes.length === 0 ? (
        <div className="flex flex-col items-center justify-center rounded-lg border border-dashed border-border py-20 text-center">
          <p className="text-sm text-muted-foreground">No signal yet. Send an event to see metrics appear here.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3">
          {eventTypes.map(eventType => (
            <Card key={eventType}>
              <CardHeader>
                <CardTitle className="flex items-center gap-2 font-mono text-xs tracking-widest text-muted-foreground uppercase">
                  <span className="h-1.5 w-1.5 shrink-0 rounded-full bg-primary" />
                  {eventType}
                </CardTitle>
              </CardHeader>
              <CardContent>
                <MetricsChart eventType={eventType} windows={windows[eventType]} />
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
