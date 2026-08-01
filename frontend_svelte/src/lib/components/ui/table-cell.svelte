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

<td data-slot="table-cell" class={cn('p-2 align-middle whitespace-nowrap', className)} {...rest}>
	{@render children?.()}
</td>
