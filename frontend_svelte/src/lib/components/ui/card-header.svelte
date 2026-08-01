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

<div data-slot="card-header" class={cn('grid auto-rows-min items-start gap-1 px-6', className)} {...rest}>
	{@render children?.()}
</div>
