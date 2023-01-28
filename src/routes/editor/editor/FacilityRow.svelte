<script lang="ts">
	import FacilityIcon from '$lib/components/FacilityIcon.svelte';
	import facilities from '$lib/data/facilities.json';
	import { tooltip } from '$lib/tooltip';
	import type { FacilityName } from '$lib/types';
	export let kind: FacilityName;

	let facility = facilities[kind];
</script>

<div
	class="container"
	style:border-left="1em solid {facility.color}"
	style:background-color={facility.color}
>
	<div class="edge" style:background-color={facility.color} use:tooltip={facility.name}>
		<FacilityIcon {kind} size={24} />
	</div>
</div>

<style>
	.container {
		display: flex;
	}
</style>
