<script lang="ts">
	import FacilityRow from './FacilityRow.svelte';
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';
	export let kind: FacilityName;

	let { color } = facilities[kind];

	function hexToRGBA(hex: string, alpha: number): string {
		let r = parseInt(hex.slice(1, 3), 16),
			g = parseInt(hex.slice(3, 5), 16),
			b = parseInt(hex.slice(5, 7), 16);

		return `rgb(${r}, ${g}, ${b}, ${alpha})`;
	}
</script>

<FacilityRow {kind} --color={color} --color-a={hexToRGBA(color, 0.5)} />
