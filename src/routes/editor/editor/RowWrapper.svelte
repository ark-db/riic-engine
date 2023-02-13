<script lang="ts">
	import FacilityRow from './FacilityRow.svelte';
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';

	export let kind: FacilityName;

	let { color } = facilities[kind];

	// Converts a hex triplet into the CSS rgb() format
	function hexToRgb(hex: string, alpha = 1.0): string {
		let r = parseInt(hex.slice(1, 3), 16),
			g = parseInt(hex.slice(3, 5), 16),
			b = parseInt(hex.slice(5, 7), 16);

		// values need to be separated by commas when passed as a style prop
		return `rgb(${r}, ${g}, ${b}, ${alpha})`;
	}
</script>

<FacilityRow {kind} --color={color} --color-a={hexToRgb(color, 0.5)} />
