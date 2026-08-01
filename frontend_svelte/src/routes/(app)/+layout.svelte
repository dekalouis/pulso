<script lang="ts">
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { getApiKey } from '$lib/api/client';
	import Nav from '$lib/components/Nav.svelte';

	let { children }: { children: Snippet } = $props();
	let ready = $state(false);

	onMount(() => {
		if (!getApiKey()) {
			goto('/login', { replaceState: true });
			return;
		}
		ready = true;
	});
</script>

{#if ready}
	<div class="flex min-h-screen bg-background text-foreground">
		<Nav />
		<main class="flex-1 overflow-y-auto p-8">
			{@render children()}
		</main>
	</div>
{/if}
