<script lang="ts">
	import { goto } from '$app/navigation';
	import { Check, Copy } from 'lucide-svelte';
	import { setApiKey } from '$lib/api/client';
	import { createTenant } from '$lib/api/tenant';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import PulseLine from '$lib/components/PulseLine.svelte';

	let key = $state('');
	let tenantName = $state('');
	let creating = $state(false);
	let createError = $state<string | null>(null);
	let createdKey = $state<string | null>(null);
	let copied = $state(false);

	function handleConnect(e: SubmitEvent) {
		e.preventDefault();
		if (!key.trim()) return;
		setApiKey(key.trim());
		goto('/');
	}

	async function handleCreate(e: SubmitEvent) {
		e.preventDefault();
		if (!tenantName.trim()) return;
		creating = true;
		createError = null;
		try {
			const res = await createTenant(tenantName.trim());
			createdKey = res.api_key;
		} catch {
			createError = 'Could not create tenant. Try again.';
		} finally {
			creating = false;
		}
	}

	function copyKey() {
		if (!createdKey) return;
		navigator.clipboard.writeText(createdKey);
		copied = true;
		setTimeout(() => (copied = false), 1500);
	}

	function continueToDashboard() {
		if (!createdKey) return;
		setApiKey(createdKey);
		goto('/');
	}
</script>

<div class="relative flex min-h-screen flex-col items-center justify-center overflow-hidden bg-background px-6">
	<PulseLine
		tiles={14}
		class="pointer-events-none absolute inset-x-0 top-1/2 h-48 w-full -translate-y-1/2 text-primary/15"
	/>

	<div class="relative z-10 flex w-full flex-col items-center text-center">
		<div class="mb-3 flex items-center gap-2 font-mono text-xs tracking-[0.3em] text-muted-foreground uppercase">
			<span class="led-live h-1.5 w-1.5 rounded-full bg-primary"></span>
			Self-hosted signal
		</div>
		<h1 class="font-display text-6xl tracking-tight text-foreground sm:text-7xl">PULSO</h1>
		<p class="mt-3 max-w-sm text-sm text-muted-foreground">
			Real-time event metrics and threshold alerts for your own infrastructure.
		</p>

		{#if createdKey}
			<Card class="mt-10 w-full max-w-sm text-left">
				<CardHeader>
					<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">
						Save your API key
					</CardTitle>
				</CardHeader>
				<CardContent class="flex flex-col gap-4">
					<p class="text-xs text-muted-foreground">
						This is shown once — store it somewhere safe. You'll need it to send events and to sign back in
						later.
					</p>
					<div class="flex items-center gap-2">
						<code class="flex-1 truncate rounded-md border border-border bg-muted px-3 py-2 font-mono text-xs text-foreground">
							{createdKey}
						</code>
						<Button variant="outline" size="icon" onclick={copyKey} aria-label="Copy API key">
							{#if copied}
								<Check size={14} class="text-primary" />
							{:else}
								<Copy size={14} />
							{/if}
						</Button>
					</div>
					<Button class="w-full" onclick={continueToDashboard}>Continue to dashboard</Button>
				</CardContent>
			</Card>
		{:else}
			<div class="mt-10 flex w-full max-w-sm flex-col gap-4">
				<Card class="text-left">
					<CardHeader>
						<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">
							Create new tenant
						</CardTitle>
					</CardHeader>
					<CardContent>
						<form onsubmit={handleCreate} class="flex flex-col gap-3">
							<Input placeholder="Tenant name" bind:value={tenantName} class="font-mono" />
							{#if createError}
								<p class="text-xs text-destructive">{createError}</p>
							{/if}
							<Button type="submit" class="w-full" disabled={creating}>
								{creating ? 'Creating…' : 'Create tenant'}
							</Button>
						</form>
					</CardContent>
				</Card>

				<Card class="text-left">
					<CardHeader>
						<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">
							Connect
						</CardTitle>
					</CardHeader>
					<CardContent>
						<form onsubmit={handleConnect} class="flex flex-col gap-3">
							<Input placeholder="API key" bind:value={key} class="font-mono" autofocus />
							<Button type="submit" class="w-full">Connect</Button>
						</form>
					</CardContent>
				</Card>
			</div>
		{/if}
	</div>
</div>
