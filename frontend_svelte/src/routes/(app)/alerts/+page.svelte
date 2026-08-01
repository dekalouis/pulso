<script lang="ts">
	import { onMount } from 'svelte';
	import { ArrowUp, ArrowDown, Trash2 } from 'lucide-svelte';
	import {
		getAlertRules,
		deleteAlertRule,
		getAlertEvents,
		createAlertRule,
		type AlertRule,
		type AlertEvent,
		type CreateAlertRuleInput
	} from '$lib/api/alerts';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Table from '$lib/components/ui/table.svelte';
	import TableHeader from '$lib/components/ui/table-header.svelte';
	import TableBody from '$lib/components/ui/table-body.svelte';
	import TableRow from '$lib/components/ui/table-row.svelte';
	import TableHead from '$lib/components/ui/table-head.svelte';
	import TableCell from '$lib/components/ui/table-cell.svelte';

	const selectClass =
		'h-9 rounded-md border border-input bg-transparent px-2.5 text-sm text-foreground outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 dark:bg-input/30';

	const th = 'font-mono text-[10px] tracking-widest text-muted-foreground uppercase';

	let rules = $state<AlertRule[]>([]);
	let events = $state<AlertEvent[]>([]);
	let form = $state<CreateAlertRuleInput>({
		event_type: '',
		rule_condition: 'above',
		threshold: 1,
		time_window: '5m'
	});

	function load() {
		getAlertRules().then((r) => (rules = r)).catch(console.error);
		getAlertEvents().then((e) => (events = e)).catch(console.error);
	}

	onMount(() => {
		load();
		const id = setInterval(load, 10_000);
		return () => clearInterval(id);
	});

	async function handleDelete(id: string) {
		rules = rules.filter((r) => r.id !== id);
		await deleteAlertRule(id);
	}

	async function handleCreate(e: SubmitEvent) {
		e.preventDefault();
		await createAlertRule(form);
		load();
	}
</script>

<div class="flex flex-col gap-6">
	<h1 class="font-display text-2xl tracking-tight text-foreground">Alerts</h1>

	<Card>
		<CardHeader>
			<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">New alert rule</CardTitle>
		</CardHeader>
		<CardContent>
			<form onsubmit={handleCreate} class="flex flex-wrap items-end gap-3">
				<div class="flex flex-col gap-1.5">
					<label for="rule-event-type" class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Event type</label>
					<Input id="rule-event-type" bind:value={form.event_type} placeholder="checkout" class="w-36 font-mono" />
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="rule-condition" class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Condition</label>
					<select id="rule-condition" bind:value={form.rule_condition} class={selectClass}>
						<option value="above">above</option>
						<option value="below">below</option>
					</select>
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="rule-threshold" class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Threshold</label>
					<Input id="rule-threshold" type="number" bind:value={form.threshold} class="w-24 font-mono" />
				</div>
				<div class="flex flex-col gap-1.5">
					<label for="rule-window" class="font-mono text-[10px] tracking-widest text-muted-foreground uppercase">Window</label>
					<select id="rule-window" bind:value={form.time_window} class={selectClass}>
						<option value="5m">5 min</option>
						<option value="15m">15 min</option>
						<option value="1h">1 hr</option>
						<option value="24h">24 hr</option>
					</select>
				</div>
				<Button type="submit">Create rule</Button>
			</form>
		</CardContent>
	</Card>

	<Card>
		<CardHeader>
			<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">Active rules</CardTitle>
		</CardHeader>
		<CardContent>
			{#if rules.length === 0}
				<p class="text-sm text-muted-foreground">No active rules yet.</p>
			{:else}
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead class={th}>Event</TableHead>
							<TableHead class={th}>Condition</TableHead>
							<TableHead class={th}>Threshold</TableHead>
							<TableHead class={th}>Window</TableHead>
							<TableHead></TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each rules as r (r.id)}
							<TableRow>
								<TableCell class="text-foreground">{r.event_type}</TableCell>
								<TableCell>
									<Badge variant="outline" class="gap-1 font-mono">
										{#if r.rule_condition === 'above'}
											<ArrowUp size={11} />
										{:else}
											<ArrowDown size={11} />
										{/if}
										{r.rule_condition}
									</Badge>
								</TableCell>
								<TableCell class="font-mono">{r.threshold}</TableCell>
								<TableCell class="font-mono text-muted-foreground">{r.time_window}</TableCell>
								<TableCell class="text-right">
									<Button
										variant="ghost"
										size="icon-sm"
										onclick={() => handleDelete(r.id)}
										class="text-muted-foreground hover:text-destructive"
									>
										<Trash2 size={14} />
									</Button>
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			{/if}
		</CardContent>
	</Card>

	<Card>
		<CardHeader>
			<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">Alert history</CardTitle>
		</CardHeader>
		<CardContent>
			{#if events.length === 0}
				<p class="text-sm text-muted-foreground">No alerts triggered yet.</p>
			{:else}
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead class={th}>Event</TableHead>
							<TableHead class={th}>Condition</TableHead>
							<TableHead class={th}>Value</TableHead>
							<TableHead class={th}>Fired</TableHead>
							<TableHead class={th}>Status</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each events as e (e.id)}
							<TableRow>
								<TableCell class="text-foreground">{e.event_type}</TableCell>
								<TableCell class="font-mono text-muted-foreground">{e.rule_condition} {e.threshold}</TableCell>
								<TableCell class="font-mono">{e.value_at_trigger}</TableCell>
								<TableCell class="font-mono text-muted-foreground">{new Date(e.triggered_at).toLocaleString()}</TableCell>
								<TableCell>
									{#if e.resolved_at}
										<Badge variant="secondary">Resolved</Badge>
									{:else}
										<Badge class="gap-1.5 border-transparent bg-amber/15 text-amber">
											<span class="led-live h-1.5 w-1.5 rounded-full bg-amber"></span>
											Firing
										</Badge>
									{/if}
								</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			{/if}
		</CardContent>
	</Card>
</div>
