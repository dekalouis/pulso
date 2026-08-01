<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';

	let {
		class: className,
		children,
		...rest
	}: HTMLAttributes<HTMLDivElement> & { children?: Snippet } = $props();
</script>

<div
	data-slot="card"
	class={cn(
		'flex flex-col gap-6 overflow-hidden rounded-xl bg-card py-6 text-sm text-card-foreground shadow-xs ring-1 ring-foreground/10',
		className
	)}
	{...rest}
>
	{@render children?.()}
</div>
