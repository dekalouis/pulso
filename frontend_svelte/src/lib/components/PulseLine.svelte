<script lang="ts">
	const TILE = 'M0,30 L18,30 L24,14 L30,30 L40,30 L46,52 L52,8 L58,30 L68,30 L74,22 L80,30 L100,30';

	let { class: className = undefined, tiles = 6 }: { class?: string; tiles?: number } = $props();
</script>

<svg viewBox={`0 0 ${100 * (tiles - 1)} 60`} preserveAspectRatio="none" class={className} aria-hidden="true">
	<g class="pulse-scroll">
		{#each Array(tiles) as _, i (i)}
			<path
				d={TILE}
				transform={`translate(${i * 100},0)`}
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		{/each}
	</g>
</svg>
