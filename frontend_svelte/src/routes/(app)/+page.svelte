<script lang="ts">
	import { onMount } from 'svelte';
	import { getMetrics, type MetricsResponse } from '$lib/api/metrics';
	import MetricsChart from '$lib/components/MetricsChart.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import { tenantName, loadTenant } from '$lib/stores/tenant';

	let data = $state<MetricsResponse | null>(null);

	let windows = $derived(data?.windows ?? {});
	let eventTypes = $derived(Object.keys(windows));

	onMount(() => {
		loadTenant();
		const load = () => getMetrics().then((d) => (data = d)).catch(console.error);
		load();
		const id = setInterval(load, 10_000);
		return () => clearInterval(id);
	});
</script>

<div class="flex flex-col gap-6">
	<div class="flex items-center justify-between">
		<h1 class="font-display text-2xl tracking-tight text-foreground">Dashboard</h1>
		<div class="flex flex-col items-end gap-1">
			{#if $tenantName}
				<span class="font-mono text-xs tracking-widest text-muted-foreground uppercase">{$tenantName}</span>
			{/if}
			<div class="flex items-center gap-2 font-mono text-xs tracking-widest text-muted-foreground uppercase">
				<span class="led-live h-1.5 w-1.5 rounded-full bg-primary"></span>
				Live
			</div>
		</div>
	</div>

	{#if eventTypes.length === 0}
		<div class="flex flex-col items-center justify-center rounded-lg border border-dashed border-border py-20 text-center">
			<p class="text-sm text-muted-foreground">No signal yet. Send an event to see metrics appear here.</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3">
			{#each eventTypes as eventType (eventType)}
				<Card>
					<CardHeader>
						<CardTitle class="flex items-center gap-2 font-mono text-xs tracking-widest text-muted-foreground uppercase">
							<span class="h-1.5 w-1.5 shrink-0 rounded-full bg-primary"></span>
							{eventType}
						</CardTitle>
					</CardHeader>
					<CardContent>
						<MetricsChart {eventType} windows={windows[eventType]} series={data?.series[eventType] ?? []} />
					</CardContent>
				</Card>
			{/each}
		</div>
	{/if}
</div>
