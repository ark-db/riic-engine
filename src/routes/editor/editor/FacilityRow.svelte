<script lang="ts">
	import FacilityIcon from '$lib/components/FacilityIcon.svelte';
	import facilities from '$lib/data/facilities.json';
	import { tooltip } from '$lib/tooltip';
	import type { FacilityName } from '$lib/types';

	export let kind: FacilityName;

	const rowOpacity = 0.7;
	let { name, color } = facilities[kind];

	// Converts a hex triplet into the CSS rgb() format
	function hexToRgb(hex: string, alpha = 1.0): string {
		let r = parseInt(hex.slice(1, 3), 16),
			g = parseInt(hex.slice(3, 5), 16),
			b = parseInt(hex.slice(5, 7), 16);

		return `rgb(${r} ${g} ${b} / ${alpha})`;
	}
</script>

<div class="container" style="--color: {color}; --color-a: {hexToRgb(color, rowOpacity)}">
	<div class="edge" use:tooltip={name}>
		<FacilityIcon {kind} size={24} />
	</div>
</div>

<style>
	.container {
		height: 5em;
		display: flex;
		background-color: var(--color-a);
	}
	.edge {
		padding: 0 0.5em;
		background-color: var(--color);
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
