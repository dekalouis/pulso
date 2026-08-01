<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { Check, Copy } from 'lucide-svelte';
	import { getApiKey, clearApiKey } from '$lib/api/client';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Button from '$lib/components/ui/button.svelte';
	import { tenantName, loadTenant } from '$lib/stores/tenant';

	let key = $state<string | null>(null);
	let copied = $state(false);

	onMount(() => {
		key = getApiKey();
		loadTenant();
	});

	function logout() {
		clearApiKey();
		goto('/login');
	}

	function copyKey() {
		if (!key) return;
		navigator.clipboard.writeText(key);
		copied = true;
		setTimeout(() => (copied = false), 1500);
	}
</script>

<div class="flex flex-col gap-6">
	<h1 class="font-display text-2xl tracking-tight text-foreground">Settings</h1>
	<Card class="max-w-lg">
		<CardHeader>
			<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">Tenant</CardTitle>
		</CardHeader>
		<CardContent>
			<p class="font-mono text-sm text-foreground">{$tenantName ?? '—'}</p>
		</CardContent>
	</Card>
	<Card class="max-w-lg">
		<CardHeader>
			<CardTitle class="font-mono text-xs tracking-widest text-muted-foreground uppercase">API key</CardTitle>
		</CardHeader>
		<CardContent class="flex flex-col gap-4">
			<div class="flex items-center gap-2">
				<code class="flex-1 truncate rounded-md border border-border bg-muted px-3 py-2 font-mono text-xs text-foreground">
					{key}
				</code>
				<Button variant="outline" size="icon" onclick={copyKey} aria-label="Copy API key">
					{#if copied}
						<Check size={14} class="text-primary" />
					{:else}
						<Copy size={14} />
					{/if}
				</Button>
			</div>
			<Button variant="destructive" onclick={logout} class="w-fit">Sign out</Button>
		</CardContent>
	</Card>
</div>
