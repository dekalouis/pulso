<script lang="ts">
	type Props = {
		eventType: string;
		windows: Record<string, number>;
		series: [number, number][];
	};

	let { eventType, windows, series }: Props = $props();

	const WINDOW_LABELS: [key: string, label: string][] = [
		['five_min', '5m'],
		['fifteen_min', '15m'],
		['one_hour', '1h'],
		['one_day', '24h']
	];

	function formatTime(ms: number) {
		return new Date(ms).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	// ponytail: fixed viewBox scaled with preserveAspectRatio="none" instead of a
	// ResizeObserver-driven responsive container — recharts' ResponsiveContainer
	// equivalent, upgrade if the chart ever needs to stay pixel-crisp on resize.
	const W = 600;
	const H = 140;
	const PAD_LEFT = 28;
	const PAD_RIGHT = 4;
	const PAD_TOP = 8;
	const PAD_BOTTOM = 20;

	let points = $derived(series.map(([ts, count]) => ({ ts, count })));
	let maxCount = $derived(Math.max(1, ...points.map((p) => p.count)));

	function x(i: number) {
		const n = points.length;
		if (n <= 1) return PAD_LEFT;
		return PAD_LEFT + (i / (n - 1)) * (W - PAD_LEFT - PAD_RIGHT);
	}
	function y(count: number) {
		const plotH = H - PAD_TOP - PAD_BOTTOM;
		return PAD_TOP + plotH - (count / maxCount) * plotH;
	}

	let linePath = $derived(
		points.map((p, i) => `${i === 0 ? 'M' : 'L'}${x(i).toFixed(1)},${y(p.count).toFixed(1)}`).join(' ')
	);
	let areaPath = $derived(
		points.length
			? `${linePath} L${x(points.length - 1).toFixed(1)},${(H - PAD_BOTTOM).toFixed(1)} L${x(0).toFixed(1)},${(H - PAD_BOTTOM).toFixed(1)} Z`
			: ''
	);

	const yTicks = $derived(
		[0, 0.5, 1].map((f) => ({ value: Math.round(maxCount * f), y: y(maxCount * f) }))
	);

	// Show ~5 evenly spaced x-axis labels.
	const xTickIndices = $derived(
		Array.from(
			new Set(
				points.length <= 1
					? points.map((_, i) => i)
					: Array.from({ length: Math.min(5, points.length) }, (_, i) =>
							Math.round((i / (Math.min(5, points.length) - 1)) * (points.length - 1))
						)
			)
		)
	);
</script>

<div class="flex flex-col gap-3">
	<svg viewBox={`0 0 ${W} ${H}`} preserveAspectRatio="none" class="h-[140px] w-full">
		<defs>
			<linearGradient id={`scope-${eventType}`} x1="0" y1="0" x2="0" y2="1">
				<stop offset="0%" stop-color="var(--phosphor)" stop-opacity="0.35" />
				<stop offset="100%" stop-color="var(--phosphor)" stop-opacity="0" />
			</linearGradient>
		</defs>

		{#each yTicks as tick, i (i)}
			<line
				x1={PAD_LEFT}
				x2={W - PAD_RIGHT}
				y1={tick.y}
				y2={tick.y}
				stroke="var(--line)"
				stroke-dasharray="2 4"
			/>
			<text x={PAD_LEFT - 6} y={tick.y} text-anchor="end" dominant-baseline="middle" font-size="10" font-family="var(--font-mono)" fill="var(--fog-solid)">
				{tick.value}
			</text>
		{/each}

		{#if points.length}
			<path d={areaPath} fill={`url(#scope-${eventType})`} stroke="none" />
			<path d={linePath} fill="none" stroke="var(--phosphor)" stroke-width="1.5" />

			{#each xTickIndices as i (i)}
				<text x={x(i)} y={H - 4} text-anchor="middle" font-size="10" font-family="var(--font-mono)" fill="var(--fog-solid)">
					{formatTime(points[i].ts)}
				</text>
			{/each}

			<circle
				cx={x(points.length - 1)}
				cy={y(points[points.length - 1].count)}
				r="3.5"
				fill="var(--phosphor)"
				class="led-live"
			/>
		{/if}
	</svg>

	<div class="flex items-center justify-between border-t border-border pt-2.5">
		{#each WINDOW_LABELS as [key, label] (key)}
			<div class="flex flex-col items-center gap-0.5">
				<span class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">{label}</span>
				<span class="font-mono text-sm text-foreground">{windows[key] ?? 0}</span>
			</div>
		{/each}
	</div>
</div>
