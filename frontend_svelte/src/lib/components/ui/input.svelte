<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';

	let {
		class: className,
		value = $bindable(),
		...rest
	}: HTMLInputAttributes = $props();
</script>

<input
	data-slot="input"
	bind:value
	class={cn(
		'h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-2.5 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30',
		className
	)}
	{...rest}
/>
