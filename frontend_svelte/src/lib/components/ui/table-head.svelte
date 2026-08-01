<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';

	let {
		class: className,
		children,
		...rest
	}: HTMLAttributes<HTMLTableCellElement> & { children?: Snippet } = $props();
</script>

<th
	data-slot="table-head"
	class={cn('h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground', className)}
	{...rest}
>
	{@render children?.()}
</th>
