<script lang="ts">
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';

	export let kind: FacilityName;
	export let level: number;

	let capacity = facilities[kind].capacity[level];
</script>

<div class="focus-template box">{capacity}</div>

<style>
	.box {
		--focus-border-offset: -1px;
		--focus-border-radius: 0;
		/* --box-width is defined in ./FacilityRow.svelte */
		width: var(--box-width);
		/* --row-height is defined in ./FacilityRow.svelte */
		height: var(--row-height);
	}
</style>
