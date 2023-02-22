<script lang="ts">
	import FacilityIcon from '$lib/components/FacilityIcon.svelte';
	import facilities from '$lib/data/facilities.json';
	import { tooltip } from '$lib/tooltip';
	import type { FacilityName } from '$lib/types';

	export let kind: FacilityName;

	let facility = facilities[kind];
</script>

<div class="container">
	<div class="edge" use:tooltip={facility.name}>
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
