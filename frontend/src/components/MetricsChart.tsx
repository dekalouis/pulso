import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, Cell } from 'recharts';

type Props = {
  eventType: string;
  windows: Record<string, number>;
};

const WINDOW_LABELS: Record<string, string> = {
  one_min: '1 min',
  five_min: '5 min',
  one_hour: '1 hr',
};

const MONO = 'IBM Plex Mono, ui-monospace, monospace';

export default function MetricsChart({ windows }: Props) {
  const chartData = Object.entries(windows).map(([key, value]) => ({
    window: WINDOW_LABELS[key] ?? key,
    count: value,
  }));

  return (
    <ResponsiveContainer width="100%" height={180}>
      <BarChart data={chartData} barSize={40}>
        <XAxis
          dataKey="window"
          stroke="#1f2d26"
          tick={{ fill: '#85988f', fontSize: 11, fontFamily: MONO }}
          tickLine={false}
        />
        <YAxis
          stroke="#1f2d26"
          tick={{ fill: '#85988f', fontSize: 11, fontFamily: MONO }}
          allowDecimals={false}
          tickLine={false}
        />
        <Tooltip
          contentStyle={{ backgroundColor: '#131e18', border: '1px solid #1f2d26', color: '#e9f2ec', fontFamily: MONO, fontSize: 12 }}
          cursor={{ fill: '#131e18' }}
        />
        <Bar dataKey="count" radius={[2, 2, 0, 0]}>
          {chartData.map((_, i) => (
            <Cell key={i} fill="#6ee7ab" />
          ))}
        </Bar>
      </BarChart>
    </ResponsiveContainer>
  );
}
