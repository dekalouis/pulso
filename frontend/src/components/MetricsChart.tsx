import { AreaChart, Area, XAxis, YAxis, Tooltip, CartesianGrid, ResponsiveContainer } from 'recharts';

type Props = {
  eventType: string;
  windows: Record<string, number>;
  series: [number, number][];
};

const WINDOW_LABELS: [key: string, label: string][] = [
  ['five_min', '5m'],
  ['fifteen_min', '15m'],
  ['one_hour', '1h'],
  ['one_day', '24h'],
];

const MONO = 'var(--font-mono)';

function formatTime(ms: number) {
  return new Date(ms).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function LiveDot({ cx, cy, index, dataLength }: { cx?: number; cy?: number; index?: number; dataLength: number }) {
  const isLast = index === dataLength - 1;
  return (
    <circle
      cx={cx}
      cy={cy}
      r={isLast ? 3.5 : 0}
      fill="var(--phosphor)"
      className={isLast ? 'led-live' : undefined}
    />
  );
}

// A live 60-minute trace, styled as a scope/vitals readout, with the
// four rolling-window counts as a compact instrument-panel readout below.
export default function MetricsChart({ eventType, windows, series }: Props) {
  const data = series.map(([ts, count]) => ({ ts, count }));
  const dataLength = data.length;

  return (
    <div className="flex flex-col gap-3">
      <ResponsiveContainer width="100%" height={140}>
        <AreaChart data={data} margin={{ top: 8, right: 4, left: -20, bottom: 0 }}>
          <defs>
            <linearGradient id={`scope-${eventType}`} x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="var(--phosphor)" stopOpacity={0.35} />
              <stop offset="100%" stopColor="var(--phosphor)" stopOpacity={0} />
            </linearGradient>
          </defs>
          <CartesianGrid vertical={false} stroke="var(--line)" strokeDasharray="2 4" />
          <XAxis
            dataKey="ts"
            tickFormatter={formatTime}
            stroke="var(--line)"
            tick={{ fill: 'var(--fog-solid)', fontSize: 10, fontFamily: MONO }}
            tickLine={false}
            axisLine={false}
            interval={14}
            minTickGap={20}
          />
          <YAxis
            stroke="var(--line)"
            tick={{ fill: 'var(--fog-solid)', fontSize: 10, fontFamily: MONO }}
            allowDecimals={false}
            tickLine={false}
            axisLine={false}
            width={26}
          />
          <Tooltip
            labelFormatter={ts => formatTime(Number(ts))}
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            formatter={((value: number) => [value, 'events']) as any}
            contentStyle={{
              backgroundColor: 'var(--panel-raised)',
              border: '1px solid var(--line)',
              color: 'var(--paper)',
              fontFamily: MONO,
              fontSize: 12,
              borderRadius: 6,
            }}
            cursor={{ stroke: 'var(--phosphor-dim)', strokeWidth: 1 }}
          />
          <Area
            type="monotone"
            dataKey="count"
            stroke="var(--phosphor)"
            strokeWidth={1.5}
            fill={`url(#scope-${eventType})`}
            isAnimationActive={false}
            dot={props => <LiveDot {...props} dataLength={dataLength} />}
          />
        </AreaChart>
      </ResponsiveContainer>

      <div className="flex items-center justify-between border-t border-border pt-2.5">
        {WINDOW_LABELS.map(([key, label]) => (
          <div key={key} className="flex flex-col items-center gap-0.5">
            <span className="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">{label}</span>
            <span className="font-mono text-sm text-foreground">{windows[key] ?? 0}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
