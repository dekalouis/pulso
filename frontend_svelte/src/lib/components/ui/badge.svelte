<script lang="ts" module>
	import { tv, type VariantProps } from 'tailwind-variants';

	export const badgeVariants = tv({
		base: 'inline-flex h-5 w-fit shrink-0 items-center justify-center gap-1 overflow-hidden rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap transition-all [&>svg]:pointer-events-none [&>svg]:size-3!',
		variants: {
			variant: {
				default: 'bg-primary text-primary-foreground',
				secondary: 'bg-secondary text-secondary-foreground',
				destructive: 'bg-destructive/10 text-destructive dark:bg-destructive/20',
				outline: 'border-border text-foreground',
				ghost: 'hover:bg-muted hover:text-muted-foreground',
				link: 'text-primary underline-offset-4 hover:underline'
			}
		},
		defaultVariants: {
			variant: 'default'
		}
	});

	export type BadgeVariant = VariantProps<typeof badgeVariants>['variant'];
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	let {
		variant = 'default',
		class: className,
		children,
		...rest
	}: {
		variant?: BadgeVariant;
		class?: string;
		children?: Snippet;
		[key: string]: unknown;
	} = $props();
</script>

<span data-slot="badge" class={cn(badgeVariants({ variant }), className)} {...rest}>
	{@render children?.()}
</span>
