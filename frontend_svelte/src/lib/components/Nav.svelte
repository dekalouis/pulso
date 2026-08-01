<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { LogOut } from 'lucide-svelte';
	import { clearApiKey } from '$lib/api/client';
	import PulseLine from './PulseLine.svelte';

	const links = [
		{ to: '/', label: 'Dashboard' },
		{ to: '/alerts', label: 'Alerts' },
		{ to: '/settings', label: 'Settings' }
	];

	function isActive(to: string) {
		return to === '/' ? page.url.pathname === '/' : page.url.pathname.startsWith(to);
	}

	function logout() {
		clearApiKey();
		goto('/login');
	}
</script>

<nav class="flex w-56 shrink-0 flex-col border-r border-sidebar-border bg-sidebar">
	<div class="border-b border-sidebar-border px-5 pt-6 pb-5">
		<span class="font-display text-lg tracking-tight text-sidebar-foreground">PULSO</span>
		<PulseLine tiles={4} class="mt-3 h-5 w-full text-primary/70" />
	</div>

	<div class="flex flex-1 flex-col gap-0.5 px-3 py-4">
		{#each links as l (l.to)}
			{@const active = isActive(l.to)}
			<a
				href={l.to}
				class={`flex items-center gap-2.5 rounded-md px-3 py-2 text-xs font-medium tracking-widest uppercase transition-colors ${
					active
						? 'bg-sidebar-accent text-sidebar-accent-foreground'
						: 'text-muted-foreground hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground'
				}`}
			>
				<span
					class={`h-1.5 w-1.5 shrink-0 rounded-full ${
						active ? 'bg-primary led-live' : 'bg-muted-foreground/30'
					}`}
				></span>
				{l.label}
			</a>
		{/each}
	</div>

	<div class="border-t border-sidebar-border px-3 py-4">
		<button
			onclick={logout}
			class="flex w-full items-center gap-2.5 rounded-md px-3 py-2 text-xs font-medium tracking-widest text-muted-foreground uppercase transition-colors hover:bg-sidebar-accent/50 hover:text-sidebar-accent-foreground"
		>
			<LogOut size={13} />
			Sign out
		</button>
	</div>
</nav>
